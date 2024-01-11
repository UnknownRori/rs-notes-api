use axum::{http::StatusCode, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct Hello {
    msg: String,
}

pub async fn hello_world() -> (StatusCode, Json<Hello>) {
    (
        StatusCode::OK,
        Json(Hello {
            msg: "Hello, World!".to_string(),
        }),
    )
}
