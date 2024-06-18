mod api;
mod hello;

use std::fmt::Display;

use anyhow::{Context, Result};
use axum::{routing::put, Router};
use surrealdb::{engine::local::Db, Surreal};
use tokio::{net::ToSocketAddrs, task::JoinHandle};

pub(crate) async fn http_server<A: ToSocketAddrs + Display>(
    bind_addr: A,
    db: Surreal<Db>,
) -> Result<JoinHandle<()>> {
    // let app_state = app::AppState::new(redis_store, schema_update_channel);

    let app = Router::new()
        .route(
            "/hello/:username",
            put(hello::api::upsert_user).get(hello::api::get_birthday),
        )
        .with_state(db);

    log::info!("Listening on {}", &bind_addr);

    let listener = tokio::net::TcpListener::bind(bind_addr)
        .await
        .context("Creating the http server listener")?;
    let server_handle = tokio::spawn(async {
        if let Err(err) = axum::serve(listener, app).await {
            log::error!("Error serving HTTP: {}", err);
        }
    });
    Ok(server_handle)
}
