use crate::responses::models::ModelsResponseGenerator;
use hyper::{Request, Response, StatusCode};

use serde_json;
use std::convert::Infallible;

pub async fn models_handler(req: Request<http_body_util::Full<hyper::body::Bytes>>) -> Result<Response<http_body_util::Full<hyper::body::Bytes>>, Infallible> {
    // Extract available models from request extensions (set by router)
    let available_models = req.extensions()
        .get::<Vec<String>>()
        .cloned()
        .unwrap_or_default();
    
    let generator = ModelsResponseGenerator::new();
    let response = generator.generate_response(&available_models);
    
    let json = serde_json::to_string(&response).unwrap_or_else(|_| {
        serde_json::to_string(&serde_json::json!({
            "error": {
                "message": "Failed to serialize models response",
                "type": "internal_error",
                "code": "serialization_error"
            }
        })).unwrap()
    });
    
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json")
        .body(http_body_util::Full::new(hyper::body::Bytes::from(json)))
        .unwrap())
}
