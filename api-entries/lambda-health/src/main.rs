use lambda_http::{run, service_fn, Error, Request, Response};
use qqself_api_entries_services::build_info;

async fn handler(req: Request) -> Result<Response<String>, Error> {
    println!("{} {}", req.method(), req.uri());
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

    #[tokio::test]
    async fn test_handler() {
        let request = lambda_http::request::from_str("{}").unwrap();
        let response = handler(request).await.unwrap();
        assert_eq!(response.status(), 200);
    }
}
