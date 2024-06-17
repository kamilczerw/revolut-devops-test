use surrealdb::{engine::local::SpeeDb, Surreal};

mod setup;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = setup::setup()?;
    let db = Surreal::new::<SpeeDb>(cli.data_dir).await?;
    db.use_ns("test").use_db("test").await?;
    println!("Hello, world!");

    Ok(())
}
