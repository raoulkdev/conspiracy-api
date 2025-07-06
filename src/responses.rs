use axum::http::{response, StatusCode};
use axum::Json;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Response {
    msg: & 'static str
}

impl Response {
    pub fn new(msg: & 'static str) -> Self {
        Self {msg}
    }
}

pub async fn hello_world() -> impl IntoResponse {
    let response = Response::new("Hello World");
    (StatusCode::OK, Json(response))
}