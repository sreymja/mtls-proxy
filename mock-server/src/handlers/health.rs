use hyper::{Request, Response, StatusCode};

use std::convert::Infallible;

pub async fn health_handler(_req: Request<http_body_util::Full<hyper::body::Bytes>>) -> Result<Response<http_body_util::Full<hyper::body::Bytes>>, Infallible> {
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(http_body_util::Full::new(hyper::body::Bytes::from(r#"{"status":"healthy","service":"mock-gpt-server"}"#)))
        .unwrap())
}
