mod cli;
mod db;
mod logger;

use clap::Parser;
pub(crate) use cli::Cli;
pub(crate) use logger::init_logger;
use surrealdb::{engine::local::Db, Surreal};

/// Initialize the application services.
pub(super) async fn setup() -> anyhow::Result<(Cli, Surreal<Db>)> {
    let cli = Cli::parse();
    init_logger(&cli)?;

    let db = db::init_db(&cli).await?;

    Ok((cli, db))
}
