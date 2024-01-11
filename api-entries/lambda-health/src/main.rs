use lambda_http::{run, service_fn, Error, Request, Response};
use qqself_api_entries_services::build_info;

async fn handler(_: Request) -> Result<Response<String>, Error> {
    Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body(build_info())
        .map_err(Error::from)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(service_fn(handler)).await
}

#[cfg(test)]
mod tests {
    use crate::handler;
    use lambda_http::{http::Request, Body};

    fn req(body: &str) -> Request<Body> {
        let fixture = r#"{"requestContext":{"http":{"method":"GET"}},"body":"[BODY]"}"#;
        let req = fixture.replace("[BODY]", body);
        lambda_http::request::from_str(&req).unwrap()
    }

    #[tokio::test]
    async fn test_handler() {
        let response = handler(req("")).await.unwrap();
        assert_eq!(response.status(), 200);
    }
}
