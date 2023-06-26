use crate::{
    storage::{account::AccountStorage, payload_storage::PayloadStorage},
    time::TimeProvider,
};

use super::HttpErrorType;

use actix_web::{
    web::{self, Data, ServiceConfig},
    HttpResponse, Responder,
};
use futures::{StreamExt, TryStreamExt};
use qqself_core::{
    binary_text::BinaryToText,
    date_time::{datetime::Duration, timestamp::Timestamp},
    encryption::{
        keys::PublicKey,
        payload::{Payload, PayloadBytes, PayloadError, PayloadId},
        tokens::{DeleteToken, SearchToken},
    },
};

const MAX_PAYLOAD_AGE: Duration = Duration::new(1, 0);

async fn health() -> impl Responder {
    "OK"
}

async fn set(
    req: String,
    payload_storage: Data<Box<dyn PayloadStorage + Sync + Send>>,
    account_storage: Data<Box<dyn AccountStorage + Sync + Send>>,
    time: Data<Box<dyn TimeProvider + Sync + Send>>,
) -> Result<impl Responder, HttpErrorType> {
    let now = time.now().await;
    let payload = validate_payload(req, now).await?;
    validate_account(&account_storage, payload.public_key()).await?;
    let payload_id = save_payload(&payload_storage, payload, now).await?;
    Ok(payload_id.to_string())
}

async fn find(
    req: String,
    payload_storage: Data<Box<dyn PayloadStorage + Sync + Send>>,
    account_storage: Data<Box<dyn AccountStorage + Sync + Send>>,
    time: Data<Box<dyn TimeProvider + Sync + Send>>,
) -> Result<HttpResponse, HttpErrorType> {
    let now = time.now().await;
    let search_token = validate_search_token(req, now)?;
    validate_account(&account_storage, search_token.public_key()).await?;
    let items = payload_storage
        .find(
            search_token.public_key(),
            search_token
                .last_known_id()
                .to_owned()
                .and_then(|v| v.decode()),
        )
        .map_err(|_| HttpErrorType::IOError("Streaming error".to_string()))
        .map(|v| {
            v.map(|(payload_id, payload_bytes)| {
                web::Bytes::from(format!("{}:{}\n", payload_id, payload_bytes.data()))
            })
        });
    Ok(HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(items))
}

async fn delete(
    req: String,
    payload_storage: Data<Box<dyn PayloadStorage + Sync + Send>>,
    account_storage: Data<Box<dyn AccountStorage + Sync + Send>>,
    time: Data<Box<dyn TimeProvider + Sync + Send>>,
) -> Result<impl Responder, HttpErrorType> {
    let now = time.now().await;
    let delete_token = validate_delete_token(req, now)?;
    validate_account(&account_storage, delete_token.public_key()).await?;
    if let Err(err) = payload_storage.delete(delete_token.public_key()).await {
        return Err(HttpErrorType::IOError(format!("{:#?}", err)));
    }
    Ok("")
}

async fn not_found() -> Result<String, HttpErrorType> {
    Err(HttpErrorType::NotFound)
}

async fn validate_payload(payload_data: String, now: Timestamp) -> Result<Payload, HttpErrorType> {
    let encoded = BinaryToText::new_from_encoded(payload_data)
        .ok_or_else(|| HttpErrorType::BadInput("Error validating encoded payload".to_string()))?;
    let payload_bytes = PayloadBytes::new_from_encrypted(encoded)
        .map_err(|_| HttpErrorType::BadInput("Error validating encrypted payload".to_string()))?;

    // Validation is CPU heavy and may take about 2ms, use thread pool to avoid blocking event loop
    // TODO What about SearchToken and DeleteToken, those are implicitly validated and blocks the event loop
    let payload =
        tokio::task::spawn_blocking(move || payload_bytes.validated(Some(now - MAX_PAYLOAD_AGE)))
            .await
            .map_err(|_| {
                HttpErrorType::IOError("Error calling payload verification".to_string())
            })?;
    payload.map_err(|err| match err {
        PayloadError::TimestampIsTooOld => HttpErrorType::OutdatedPayload,
        _ => HttpErrorType::BadInput(format!("Payload validation failure. {}", err)),
    })
}

fn validate_search_token(data: String, now: Timestamp) -> Result<SearchToken, HttpErrorType> {
    SearchToken::decode(data, Some(now - MAX_PAYLOAD_AGE))
        .map_err(|err| HttpErrorType::BadInput(format!("Error encoding search token. {}", err)))
}

fn validate_delete_token(data: String, now: Timestamp) -> Result<DeleteToken, HttpErrorType> {
    DeleteToken::decode(data, Some(now - MAX_PAYLOAD_AGE))
        .map_err(|err| HttpErrorType::BadInput(format!("Error encoding delete token. {}", err)))
}

async fn validate_account(
    _: &Data<Box<dyn AccountStorage + Sync + Send>>,
    _: &PublicKey,
) -> Result<(), HttpErrorType> {
    // TODO Account validation should be optimistic - always pass. Then once a day
    // get disabled accounts from DB, store it in memory and do this check for those
    // to be able to serve requests as soon as we got the payment
    Ok(())
}

async fn save_payload(
    storage: &Data<Box<dyn PayloadStorage + Sync + Send>>,
    payload: Payload,
    now: Timestamp,
) -> Result<PayloadId, HttpErrorType> {
    let payload_id = PayloadId::encode(now, payload.plaintext_hash().clone());
    if let Err(err) = storage.set(payload, payload_id.clone()).await {
        return Err(HttpErrorType::IOError(format!("{:#?}", err)));
    }
    Ok(payload_id)
}

pub fn http_config(
    payload_storage: Data<Box<dyn PayloadStorage + Send + Sync>>,
    account_storage: Data<Box<dyn AccountStorage + Send + Sync>>,
    time: Data<Box<dyn TimeProvider + Send + Sync>>,
) -> impl FnOnce(&mut ServiceConfig) {
    |cfg: &mut web::ServiceConfig| {
        cfg.app_data(
            web::JsonConfig::default()
                .error_handler(|err, _| HttpErrorType::BadInput(err.to_string()).into()),
        )
        .app_data(payload_storage)
        .app_data(account_storage)
        .app_data(time)
        .route("/health", web::get().to(health))
        .route("/set", web::post().to(set))
        .route("/find", web::post().to(find))
        .route("/delete", web::post().to(delete))
        .default_service(web::route().to(not_found));
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        http::HttpError,
        storage::{account_mem::MemoryAccountStorage, payload_storage_mem::PayloadStorageMemory},
        time::TimeStatic,
    };

    use super::*;
    use actix_web::{
        http::header,
        test,
        web::{self, Bytes, Data, ServiceConfig},
        App,
    };
    use qqself_core::encryption::{hash::StableHash, keys::PrivateKey};

    const PUBLIC_KEY_1: &str = "8A4MdxHGkuBnV4CY4W3ZgmMTiZkQHi1PdxG4yov65odytYFXkttWy8qojEp5rhNWn9ae3QWigZsfmSVojU62dFbUDR98p74VUqo47AoLLabVv7Ycj6VoEZj1Gz9YPPDhcUjbkzgzLb5n799MydJYdRLA17wDAuvNTcJ4m27F2jzg7Zv26r94eYbRRrYH6oauQGPr9a6XyvNKTzykLkU9m5C3vEnpTVai2NMdib9JiEeJUMUSaApNd4r3ZF9i46suP7qD9gimj2USuh1QHY3r9YKmcyurkZRGZhjyXAnbae98vuJtUxVyMMzV9QWkV1BodGMFc4gE77HhULKk1Z23igQWJZsDTUDhiZdLxs5pmW1699zEgNt42PtJGxQ4ouL5UZcNv42UpUrrXsnKpAKLkRKZTfpsdp4zmPYfSjMNqPQLqiyDLw1B1b5Vs23pAYNMNJoBJXp3wMsJFngqPtPDWZ9Bgm5361uAZa2yNBBfaJMoumTjAPY54MWzYbeqj7mB7ZvLm1351SVJn8rNqrHAE6fNxbruJVwjzbKzbLmD859ZBd2F1V4SKRQZSAymj9sfJYYCn3Z6KoKzBSgH2QYXoTb93dVGDGqegfwZ9EYq";
    const PRIVATE_KEY_1: &str = "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";
    const PUBLIC_KEY_2: &str = "8A4MdxHGkuBnV4CY4W3ZgmMTiZkQHi1PdxG4yov65odytYFXkttWy8qojEp5rhNWn9ae3QWigZsfmSVojU62dFbUDR98p74VUqEKPRmocEuhWJthsJJ44rdBUxeWKdcUfKFdThfJ5N9ZL41CYjJWQahFqkf6KcZcKMrzLwuvvNL3PQJ7Ly688RrUdYrY8NozzkiK1rUcBZCWAcXvavpDkxh46j7si82S1oZmpkfVT77nkwdHhUYNMmSxwnjQj3iTDqVYJqext2ZM11TMsr5FSfQrr1WUv4ZLyjMkE7uVHsJih3kB4dkfg5LYJfUfnGw9fDfRyCk6YFQ7t2HxTJRxigMxACSJpbCCgza2iA1LdEA8rYCKnLWsCeXMZC4fezu7Vhb2ivuYYRc16vkcJKq4xKxhp6mXWP5jrY9Y2dMWrMrBVeiE8ac1Z6D1EdvanJUzHisyRXg3K2DKmrvzKer7f67pXjHYUnLh9s3owzULxGsCZ88BLmMgrw5JrTvbYoRhACaUksFTUhJxAHJAysVDVHSr4PEhd6oqzPoM4TqFLYmaa5aYGBL8NgqJarvWvM59finsXpVvi2zcZ7hbFGHE2vLjqZpyjeiJNBc6DyEh";
    const PRIVATE_KEY_2: &str = "3BNsUkQvBp5DeuJKGrLKKaWe4gmNyD4MiWyw7NbUUyjBVdHawEXJhsuUQBgx7wnkQ4Au25PsunzP4dSDEFaaFiA54UU4X4uS1k7Rdau2W2vz8vN5XhgM4NFhapFu8wqRzr8khPQdkuxpa3U4VooBsLvHmSd4LrfnuALZWpb4ZSrCwBf5EgkwuzPj9cxM24hWueBUqZkWuVCbWWSBKsVpShnWUZhxW63kBtFkHw1dJDnvY6owoai8YPoNDoAxvpzcksfmE7EAxRU1uGBmsTGmTroGrzPPBLc3WTCWsu1bb1RTx9YrcKubfgCzavr29x8dtKWUV3mLENo3HyoWea1WCHFjoSrQLXFdaXvLjKxJfxNi3W919XrnvkFHj2RgL7STsnWGRtkBfGwFn1jWHp6u4xUwuSazxxQB6GpATzP1agtZFzDCKUJjnvK27TpXzBzkG4wPTSm12MMZJRgaJihne7ZC7YrLa2AbH6Hx7gKemZoJUTg7L9mm6x66W5DPeVYpySPjcVJG5n4qLxECbcuTbCcdeTcf4aQFtN42uBXrdAdtxWaW4s9DFJkVwpZAKD5VKsaHQcWJdKgGG62dXJ3hu6C3iWnTzerZQFqD7ZdkoMPYrgkxUMPXbRv6maRo3UD8fxCNv96bJWmsPKpWS74QXctwTMjv18Sss1n8LgLTNs8ZkYgC4JbmmUaHQxuK3tAiNuDjoiy3kt1KfLzFMtbCEiboginJ86mzc9M8dbZBMC9ctVkvCA5yzMrdF74cpthMGdB9NFbF9hKzmG6hA7JEGJ7VXPnf7PXWyf4NSwKC8vMDGvAbg4Tru5rYvaoqPFfGXwEYPSDGshsgeA3BcNAeLXk1A6GzKXSfsRdDaveLL85eGEu9ptAGyx4AhbxN5n18cDR296ppqEwoLKN1w4D5MS8KV7V4xDjNMjiLfEnqA3ePzA7GvEpVom74A1tRk39L6r5uL2FQtn4WD8YgxCUhLmyAdQS6ZrVhbVjuu3xSUgXHQDB23j5RCsLDJt1JyTjjY5QbsQ8N4JfzmquEbRc89bKkiuqXeoCU6EC5PXYR4KpXMSmVxoonEG8ZdK5jPaxczDqdMxeyZKP2mFKiCo9gd9wRMnQANB7B5YwsaJ5Q3qDQPmEDbtEVZ9pkkbZsjigYeDYbAJw1CikSTzDQWhJqS4UhW856bq9JGn34KTSSjPcV77mKiNPFZw4h5fpUWvfRDY9eYFDJrUxYFm6NafZBjbpjp9NttCScMM8ciMACrhKFFiQBoikoh2TvGJ9TmJsfhhwmFZmwhxd2eJ9VZSvuemwLbAcMuLGiqpHNUWAPCX8svr2JLMjsYqADxu3f43UHiRqsoAVjhcWhpk5fFWSGxH2cWYcsrwXjr6dPzEphCKsPCAUaEqWvNs7myx9pW1NdQ7RU22cyf9WDfkHS7Z5kq6ejvS78CcneCP9ZTgaAhxSJCagVt4Ee98r4pZRfm9YFh1h252PuFGLKi3s96UJAfYMdgpYWUJE94FRtcwjJfo7USPVXxRP71DeGMMSbtdD46VEEzsu7FRd5TG4ZHGnTKSyj12RNZQm18c9QqKCywEF1Y5oL5jsYaHZfeUXpAxsd1QDbFMwtQEiNUN84VWBTqUdvSvNt22fBd8BM9zk8Gquw6kKYEE4FuxrDrGU3a6WyQQfLmrW9JHqijJPnioMXqnWhyV14Ks5kk1D3VcxNzz7wzBBqpMLtRz8GxBrbCWSuCTCmcmdcQGgksLAxXkmoCEESugj23VCdT6Kt3KvjHrfKygpxKW5XE9jWsqVGkYMVJuqWgRnd4M87v4mxpeUgeiwkkQkfX29Qmvco2y7XzQMSi9Bdxz6DEEuNoD3c6D61Y8uybavPaWXRkA9yytf27ssweLaUAxQ5ZDixLw2jpwQ9D3XNaDVQkLcvKC3PUKUhy84GjuSk9H5c9ubr9Zmfv2ZfMVMp3AsrG6f4N7kwcpKmFqCHvWX5xrVTzdGdcX8NXBb9432ZcMBqaHNpmSbyFNKvNTEMD6RdRVvsujbAvq6m6apaz3zmRZz3Zg5BPnXDaRMmmbHUhr3GnqHGNDz7D7pzA8CSG1JGBLiMAB7fQJUJcT5uWtyBw6ct9r6AeWu6ezRzp6ueZYRze5Nb4cmDiUnMcAJ8rRddpZzHwcGGEY3FpAtfyDmvvNxy1GDFDdnzfvasYW83oTSt4cNcBZE1aJTgUjMvgJETFSGemdr964d6BDnL4dWLzZgvngpAo1Y1GmmjkQBBFYbMKyezuU2JRSxx3phNp7NprpoVzvGYAkTHDJSQgNiLjEMuaZAhaMMz5oMRDFKfuRS6zVFwzy";

    fn keys(public: &str, private: &str) -> (PublicKey, PrivateKey) {
        let public_key =
            PublicKey::new(BinaryToText::new_from_encoded(public.to_string()).unwrap()).unwrap();
        let private_key =
            PrivateKey::new(BinaryToText::new_from_encoded(private.to_string()).unwrap()).unwrap();
        (public_key, private_key)
    }

    type Payload = Data<Box<dyn AccountStorage + Send + Sync>>;
    type Account = Data<Box<dyn PayloadStorage + Send + Sync>>;
    type Time = Data<Box<dyn TimeProvider + Send + Sync>>;
    // That madness caused by inability to return actix_web:App and passing function to `App.configure` is a recommended way
    // It's fine for configuring an app, but we want to share entry/account storages in tests so this beast was created
    // Essentially lazy_static should have been a perfect fit here to share storages across the tests but that didn't work with some weird errors
    fn test_app() -> (Payload, Account, Time, impl FnOnce(&mut ServiceConfig)) {
        let entry_storage = Data::new(
            Box::new(PayloadStorageMemory::new()) as Box<dyn PayloadStorage + Send + Sync>
        );
        let account_storage = Data::new(
            Box::new(MemoryAccountStorage::new()) as Box<dyn AccountStorage + Send + Sync>
        );
        let time =
            Data::new(Box::new(TimeStatic::new(1662750865)) as Box<dyn TimeProvider + Send + Sync>);
        (
            account_storage.clone(),
            entry_storage.clone(),
            time.clone(),
            |cfg: &mut web::ServiceConfig| http_config(entry_storage, account_storage, time)(cfg),
        )
    }

    fn req_set(body: String) -> test::TestRequest {
        test::TestRequest::post()
            .uri("/set")
            .insert_header((header::CONTENT_TYPE, "text/plain"))
            .set_payload(body)
    }

    fn req_delete(body: String) -> test::TestRequest {
        test::TestRequest::post()
            .uri("/delete")
            .insert_header((header::CONTENT_TYPE, "text/plain"))
            .set_payload(body)
    }

    fn req_find(body: String) -> test::TestRequest {
        test::TestRequest::post()
            .uri("/find")
            .insert_header((header::CONTENT_TYPE, "text/plain"))
            .set_payload(body)
    }

    async fn items_plaintext(
        payload_storage: &Data<Box<dyn PayloadStorage + Send + Sync>>,
        public_key: &PublicKey,
        private_key: &PrivateKey,
    ) -> Vec<String> {
        payload_storage
            .find(public_key, None)
            .map(|v| v.unwrap())
            .map(|(_, data)| data.validated(None).unwrap())
            .map(|v| v.decrypt(private_key).unwrap())
            .collect::<Vec<_>>()
            .await
    }

    #[actix_web::test]
    async fn test_health() {
        let (_, _, _, configure) = test_app();
        let init_service = test::init_service(App::new().configure(configure)).await;
        let app = init_service;
        let req = test::TestRequest::get().uri("/health").to_request();
        let resp = test::call_and_read_body(&app, req).await;
        assert_eq!(resp, Bytes::from_static(b"OK"))
    }

    #[actix_web::test]
    async fn test_set_bad_json() {
        let (_, _, _, configure) = test_app();
        let app = test::init_service(App::new().configure(configure)).await;
        let req = test::TestRequest::post()
            .uri("/set")
            .insert_header((header::CONTENT_TYPE, "application/json"))
            .set_payload("{}")
            .to_request();
        let resp: HttpError = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.error_code, 422);
        assert_eq!(resp.error, "BadInput. Error validating encoded payload");
    }

    #[actix_web::test]
    async fn test_not_found() {
        let (_, _, _, configure) = test_app();
        let app = test::init_service(App::new().configure(configure)).await;
        let req = test::TestRequest::post().uri("/invalid").to_request();
        let resp: HttpError = test::call_and_read_body_json(&app, req).await;
        assert_eq!(resp.error_code, 404);
        assert_eq!(resp.error, "Requested endpoint not found")
    }

    #[actix_web::test]
    async fn test_too_old_payload() {
        let (_, _, _, configure) = test_app();
        let app = test::init_service(App::new().configure(configure)).await;
        let (public_key, private_key) = keys(PUBLIC_KEY_1, PRIVATE_KEY_1);
        let encrypted = PayloadBytes::encrypt(
            &public_key,
            &private_key,
            Timestamp::default(),
            "entry",
            None,
        )
        .unwrap();
        let resp: HttpError =
            test::call_and_read_body_json(&app, req_set(encrypted.data()).to_request()).await;
        assert_eq!(resp.error, "OutdatedPayload. Payload was created too long time ago - create a new one with up to date timestamp, check /info endpoint for server timestamp".to_string());
        assert_eq!(resp.error_code, 408);
    }

    #[actix_web::test]
    async fn test_set_bad_input() {
        let (_, _, _, configure) = test_app();
        let app = test::init_service(App::new().configure(configure)).await;
        let (public_key, private_key) = keys(PUBLIC_KEY_1, PRIVATE_KEY_1);
        let encrypted =
            PayloadBytes::encrypt(&public_key, &private_key, Timestamp::now(), "entry", None)
                .unwrap();
        let resp: HttpError =
            test::call_and_read_body_json(&app, req_set(encrypted.data() + "h").to_request()).await;
        assert_eq!(
            resp.error,
            "BadInput. Payload validation failure. Cannot read binary data".to_string()
        );
        assert_eq!(resp.error_code, 422);
    }

    #[actix_web::test]
    async fn test_set_add() {
        let (_, payload_storage, time, configure) = test_app();
        let app = test::init_service(App::new().configure(configure)).await;
        let (public_key, private_key) = keys(PUBLIC_KEY_1, PRIVATE_KEY_1);
        for (ts, expected) in [
            (1, b"00000000001662750866|SWxyLukYqS63bYMvfwoj8f"),
            (3, b"00000000001662750869|R85af9xML6WR7fNUXNgi5V"),
            (2, b"00000000001662750871|93i31rxkhgVVzHahAA2LBF"),
        ] {
            time.sleep(ts).await;
            let encrypted = PayloadBytes::encrypt(
                &public_key,
                &private_key,
                time.now().await,
                &ts.to_string(),
                None,
            )
            .unwrap();
            let resp = test::call_service(&app, req_set(encrypted.data()).to_request()).await;
            assert_eq!(resp.status(), 200);
            assert_eq!(test::read_body(resp).await.to_vec(), expected);
        }
        let got = items_plaintext(&payload_storage, &public_key, &private_key).await;
        assert_eq!(got, vec!["1", "3", "2"])
    }

    #[actix_web::test]
    async fn test_set_replace() {
        let (_, payload_storage, _, configure) = test_app();
        let app = test::init_service(App::new().configure(configure)).await;
        let (public_key, private_key) = keys(PUBLIC_KEY_1, PRIVATE_KEY_1);
        let mut replace_id = None;
        // 2 will be replaced with 4
        for ts in [1, 2, 3, 4] {
            let previous_version = if ts == 4 {
                replace_id.as_ref().cloned()
            } else {
                None
            };
            let encrypted = PayloadBytes::encrypt(
                &public_key,
                &private_key,
                Timestamp::from_u64(Timestamp::now().as_u64() + ts),
                &ts.to_string(),
                previous_version,
            )
            .unwrap();
            let payload = encrypted.validated(None).unwrap();
            let resp = test::call_service(&app, req_set(payload.data().data()).to_request()).await;
            assert_eq!(resp.status(), 200);
            if ts == 2 {
                let body = test::read_body(resp).await.to_vec();
                let body = String::from_utf8(body).unwrap();
                let payload_id = PayloadId::new_encoded(body);
                replace_id.replace(payload_id);
            }
        }
        let got = items_plaintext(&payload_storage, &public_key, &private_key).await;
        assert_eq!(got, vec!["1", "3", "4"])
    }

    #[actix_web::test]
    async fn test_find() {
        let (_, _, time, configure) = test_app();
        let app = test::init_service(App::new().configure(configure)).await;
        let (public_key, private_key) = keys(PUBLIC_KEY_1, PRIVATE_KEY_1);
        let time_start = time.now().await;
        for ts in [1, 2, 3] {
            time.sleep(ts).await;
            let encrypted = PayloadBytes::encrypt(
                &public_key,
                &private_key,
                Timestamp::from_u64(time_start.as_u64() + ts),
                &ts.to_string(),
                None,
            )
            .unwrap();
            let resp = test::call_service(&app, req_set(encrypted.data()).to_request()).await;
            assert_eq!(resp.status(), 200)
        }
        let extract_plaintext = |data: Bytes| {
            let s = String::from_utf8(data.to_vec()).unwrap();
            let mut output = Vec::new();
            for s in s.lines() {
                let payload_start_pos = s.find(':').unwrap() + 1;
                let binary_text =
                    BinaryToText::new_from_encoded(s[payload_start_pos..].to_string()).unwrap();
                let bytes = PayloadBytes::new_from_encrypted(binary_text).unwrap();
                let valid = bytes.validated(None).unwrap();
                let plaintext = valid.decrypt(&private_key).unwrap();
                output.push(plaintext);
            }
            output
        };

        // Return all
        let body = SearchToken::encode(&public_key, &private_key, time_start, None).unwrap();
        let resp = test::call_and_read_body(&app, req_find(body).to_request()).await;
        assert_eq!(extract_plaintext(resp), vec!["1", "2", "3"]);

        // Return after
        let body = SearchToken::encode(
            &public_key,
            &private_key,
            time.now().await,
            Some(PayloadId::encode(
                Timestamp::from_u64(time_start.as_u64() + 2),
                StableHash::hash_string("s"),
            )),
        )
        .unwrap();
        let resp = test::call_and_read_body(&app, req_find(body).to_request()).await;
        assert_eq!(extract_plaintext(resp), vec!["2", "3"]);

        // Another key
        let (public_key, private_key) = keys(PUBLIC_KEY_2, PRIVATE_KEY_2);
        let body = SearchToken::encode(&public_key, &private_key, time_start, None).unwrap();
        let resp = test::call_and_read_body(&app, req_find(body).to_request()).await;
        assert!(extract_plaintext(resp).is_empty());
    }

    // TODO For now this test acts as after all cleanup procedure, better make it explicit and more reliable
    #[actix_web::test]
    async fn test_delete() {
        for (public_key, private_key) in &[
            keys(PUBLIC_KEY_1, PRIVATE_KEY_1),
            keys(PUBLIC_KEY_2, PRIVATE_KEY_2),
        ] {
            let (_, payload_storage, _, configure) = test_app();
            let app = test::init_service(App::new().configure(configure)).await;
            let time_start = Timestamp::now();
            let encrypted = PayloadBytes::encrypt(
                public_key,
                private_key,
                Timestamp::from_u64(time_start.as_u64()),
                "foo",
                None,
            )
            .unwrap();
            let resp = test::call_service(&app, req_set(encrypted.data()).to_request()).await;
            assert_eq!(resp.status(), 200);
            let body = DeleteToken::encode(public_key, private_key, time_start).unwrap();
            test::call_and_read_body(&app, req_delete(body).to_request()).await;
            let got = items_plaintext(&payload_storage, public_key, private_key).await;
            assert!(got.is_empty());
        }
    }
}
