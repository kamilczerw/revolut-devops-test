use crate::app::Store;
use anyhow::Result;
use surrealdb::{engine::local::SpeeDb, Surreal};

use super::Cli;

/// Initialize the database.
pub(super) async fn init_db(cli: &Cli) -> Result<Store> {
    let db = Surreal::new::<SpeeDb>(cli.data_dir.clone()).await?;
    db.use_ns("revolut").use_db("revolut").await?;

    let store = Store::new(db);

    Ok(store)
}
