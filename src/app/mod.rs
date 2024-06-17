mod hello;

use std::fmt::Display;

use anyhow::Result;
use axum::{routing::get, Router};
use surrealdb::{engine::local::Db, Surreal};
use tokio::{net::ToSocketAddrs, task::JoinHandle};

pub(crate) async fn http_server<A: ToSocketAddrs + Display>(
    bind_addr: A,
    db: Surreal<Db>,
) -> Result<JoinHandle<()>> {
    // let app_state = app::AppState::new(redis_store, schema_update_channel);

    let app = Router::new()
        .route("/hello/:username", get(hello::api::get_birthday))
        // .route("/api/v1/schemas", post(app::schema::api::create_schema))
        // .route(
        //     "/api/v1/gamedata/:schema_name/:id",
        //     get(app::gamedata::api::get_gamedata),
        // )
        // .route("/fetch/:data_type/:class", get(retrieve_data_with_type))
        // .route("/:class/data", get(retrieve_data))
        .with_state(db);

    log::info!("Listening on {}", &bind_addr);

    let listener = tokio::net::TcpListener::bind(bind_addr).await.unwrap();
    let server_handle = tokio::spawn(async {
        axum::serve(listener, app).await.unwrap();
    });
    Ok(server_handle)
}
