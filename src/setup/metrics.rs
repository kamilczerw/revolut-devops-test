use prometheus::{register_counter_vec, CounterVec, Gauge, HistogramVec};

use lazy_static::lazy_static;
use prometheus::{labels, opts, register_gauge, register_histogram_vec};

lazy_static! {
    pub static ref HTTP_COUNTER: CounterVec = register_counter_vec!(
        opts!(
            "example_http_requests_total",
            "Number of HTTP requests made.",
        ),
        &["endpoint", "code", "method"]
    )
    .unwrap();
    pub static ref HTTP_BODY_GAUGE: Gauge = register_gauge!(opts!(
        "example_http_response_size_bytes",
        "The HTTP response sizes in bytes.",
        labels! {"handler" => "all",}
    ))
    .unwrap();
    pub static ref HTTP_REQ_HISTOGRAM: HistogramVec = register_histogram_vec!(
        "example_http_request_duration_seconds",
        "The HTTP request latencies in seconds.",
        &["endpoint"]
    )
    .unwrap();
}
