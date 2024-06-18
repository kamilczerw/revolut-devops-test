use axum::{body::Body, http::StatusCode, response::IntoResponse, Json};
use prometheus::{Encoder, TextEncoder};
use serde_json::json;

/// Serve the Prometheus metrics.
pub async fn metrics() -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();

    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    (StatusCode::OK, Body::from(buffer))
}

/// Health check endpoint.
pub async fn health() -> impl IntoResponse {
    (StatusCode::OK, Json(json!({"status": "ok"})))
}
