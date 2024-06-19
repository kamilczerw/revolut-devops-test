use axum::{extract::Request, middleware::Next, response::Response};

use crate::setup::metrics::{HTTP_COUNTER, HTTP_REQ_HISTOGRAM};

pub async fn metrics(request: Request, next: Next) -> Response {
    let endpoint = request.uri().path().to_string();
    let method = &request.method().to_string();

    let timer = HTTP_REQ_HISTOGRAM
        .with_label_values(&[&endpoint, method])
        .start_timer();

    // Call the next middleware in the chain.
    let response = next.run(request).await;

    let status_code = response.status().to_string();

    HTTP_COUNTER
        .with_label_values(&[&endpoint, &status_code, method])
        .inc();

    timer.observe_duration();

    response
}
