use anyhow::{Context, Result};
use axum::{
    extract::Request,
    http::HeaderName,
    middleware::{self, Next},
    response::Response,
    routing::{get, put},
    Router,
};
use rand::Rng;
use std::{fmt::Display, time::Duration};
use tokio::signal;
use tokio::{net::ToSocketAddrs, task::JoinHandle};
use tower::ServiceBuilder;
use tower_http::{
    request_id::{MakeRequestId, PropagateRequestIdLayer, RequestId, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    ServiceBuilderExt,
};

use crate::app::{health, hello, Store};

/// The header name for the request ID used for tracing.
static X_REQUEST_ID: HeaderName = HeaderName::from_static("x-request-id");

/// Create HTTP servers for serving external requests as well as the health requests.
/// The servers are split so the health port doesn't get exposed to the external users.
///
/// # Args
///
/// - `bind_addr`: The address to bind the external HTTP server to
/// - `health_bind_addr`: The address to bind the health server to
/// - `db`: The storage object that will be passed to the axum server and can be
///         later accessed in the request handlers
///
/// # Returns
///
/// A tuple of handles to the servers.
/// The caller is responsible for waiting for the servers to finish.
pub(crate) async fn http_server<A: ToSocketAddrs + Display>(
    bind_addr: A,
    health_bind_addr: A,
    db: Store,
) -> Result<(JoinHandle<()>, JoinHandle<()>)> {
    let app = Router::new()
        .route(
            "/hello/:username",
            put(hello::api::upsert_user).get(hello::api::get_birthday),
        )
        .layer(
            ServiceBuilder::new()
                .set_x_request_id(RandomRequestId::default())
                .layer(SetRequestIdLayer::new(
                    X_REQUEST_ID.clone(),
                    RandomRequestId::default(),
                ))
                // Inject the request ID into the MDC.
                .layer(middleware::from_fn(mdc_injector))
                // propagate `x-request-id` headers from request to response
                .layer(PropagateRequestIdLayer::new(X_REQUEST_ID.clone()))
                .layer(
                    TraceLayer::new_for_http()
                        .make_span_with(DefaultMakeSpan::new().include_headers(true))
                        .on_response(DefaultOnResponse::new().include_headers(true)),
                )
                .layer(middleware::from_fn(health::middleware::metrics))
                .propagate_x_request_id(),
        )
        .with_state(db);

    let health_app = Router::new()
        .route("/metrics", get(health::api::metrics))
        .route("/health", get(health::api::health));

    log::info!("Listening http server on {}", &bind_addr);
    let server_handle = create_server(bind_addr, app).await?;

    log::info!("Listening health server on {}", &health_bind_addr);
    let health_handle = create_server(health_bind_addr, health_app).await?;

    // TODO: Write loadtests using Goose.

    Ok((server_handle, health_handle))
}

/// Create a new HTTP server.
///
/// # Args
///
/// - `bind_addr`: The address to bind the server to
/// - `app`: The application router
///
/// # Returns
///
/// A handle to the server.
/// The caller is responsible for waiting for the server to finish.
///
/// # Example
///
/// ```rust
/// let metrics = Router::new()
///   .route("/metrics", get(health::api::metrics));
/// let health = Router::new()
///   .route("/health", get(health::api::health));
///
/// let metrics_handle = create_server("[::1]:4200", metrics).await?;
/// let health_handle = create_server("[::1]:4300", health).await?;
///
/// tokio::select! {
///   _ = metrics_handle => log::info!("HTTP server shutdown."),
///   _ = health_handle => log::info!("HTTP server shutdown."),
/// }
/// ```
async fn create_server<A: ToSocketAddrs + Display>(
    bind_addr: A,
    app: Router,
) -> Result<JoinHandle<()>> {
    // Add a timeout layer to let the application close the connections gracefully.
    let app = app.layer(TimeoutLayer::new(Duration::from_secs(10)));

    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .context("Creating the http server listener")?;

    let server_handle = tokio::spawn(async {
        if let Err(err) = axum::serve(listener, app)
            .with_graceful_shutdown(shutdown_signal())
            .await
        {
            log::error!("Error serving HTTP: {}", err);
        }
    });
    Ok(server_handle)
}

async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

/// Tower middleware that injects the request ID into the MDC.
/// The request ID will be printed in all log messages for the duration of the request.
/// The pattern for printing the request ID in logs is `{X(request_id)}`
pub async fn mdc_injector(request: Request, next: Next) -> Response {
    let request_id = request.extensions().get::<RequestId>().unwrap();
    let request_id = format!("{:?}", request_id);
    log_mdc::insert("request_id", request_id);

    // Call the next middleware in the chain.
    let response = next.run(request).await;

    // Clear the MDC after the response is generated.
    log_mdc::clear();

    response
}

// A `MakeRequestId` that increments an atomic counter
#[derive(Clone, Default)]
struct RandomRequestId();

impl MakeRequestId for RandomRequestId {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        let mut rng = rand::thread_rng();
        let request_id = rng.gen::<u64>().to_string().parse().unwrap();

        Some(RequestId::new(request_id))
    }
}
