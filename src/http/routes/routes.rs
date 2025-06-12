use crate::config::AppConfig;
use axum::{Json, Router, response::IntoResponse, routing::get};
use chrono::Utc;
use serde::Serialize;

#[derive(Serialize)]
struct HealthResponse {
    status: &'static str,
    timestamp: String,
}

async fn health_check() -> impl IntoResponse {
    let response = HealthResponse {
        status: "OK",
        timestamp: Utc::now().to_rfc3339(),
    };

    Json(response)
}

pub fn create_routes(_config: AppConfig) -> Router {
    Router::new().route("/health", get(health_check))
}
