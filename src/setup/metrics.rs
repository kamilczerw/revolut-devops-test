use prometheus::{register_counter_vec, CounterVec, HistogramVec};

use lazy_static::lazy_static;
use prometheus::{opts, register_histogram_vec};

lazy_static! {
    pub static ref HTTP_COUNTER: CounterVec = register_counter_vec!(
        opts!("http_requests_total", "Number of HTTP requests made.",),
        &["endpoint", "code", "method"]
    )
    .unwrap();
    pub static ref HTTP_REQ_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "http_request_duration_seconds",
        "The HTTP request latencies in seconds.",
        &["endpoint", "method"]
    )
    .unwrap();
}
