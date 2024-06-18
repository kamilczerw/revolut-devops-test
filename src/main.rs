mod app;
mod setup;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (cli, store) = setup::setup().await?;
    // Setup the HTTP server.
    let http_server = app::http_server(cli.bind_addr, store).await?;

    // Wait for the HTTP server to stop.
    http_server.await?;

    Ok(())
}
