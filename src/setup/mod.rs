mod cli;
mod db;
pub(crate) mod http;
mod logger;
pub mod metrics;

use clap::Parser;
pub(crate) use cli::Cli;
pub(crate) use logger::init_logger;

use crate::app::Store;

/// Initialize the application services.
pub(super) async fn setup() -> anyhow::Result<(Cli, Store)> {
    let cli = Cli::parse();
    init_logger(&cli)?;

    let db = db::init_db(&cli).await?;

    Ok((cli, db))
}
