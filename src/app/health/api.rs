use axum::{body::Body, http::StatusCode, response::IntoResponse};
use prometheus::{Encoder, TextEncoder};

pub async fn metrics() -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();

    let mut buffer = vec![];
    encoder.encode(&metric_families, &mut buffer).unwrap();

    (StatusCode::OK, Body::from(buffer))
}
