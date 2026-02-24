use axum::{Json, Router, response::IntoResponse, routing::get};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub version: Option<String>,
}

impl HealthResponse {
    pub fn healthy() -> Self {
        Self {
            status: "healthy".to_string(),
            version: Some(env!("CARGO_PKG_VERSION").to_string()),
        }
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/health",
    tag = "Health",
    responses(
        (status = 200, description = "Service is healthy", body = HealthResponse)
    )
)]
pub async fn check() -> impl IntoResponse {
    Json(HealthResponse::healthy())
}

pub fn router() -> Router {
    Router::new().route("/api/v1/health", get(check))
}
