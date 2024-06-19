mod app;
mod setup;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize all the services required by the application.
    let (cli, store) = setup::setup().await?;

    // Setup the HTTP servers.
    let (http_server, health_server) =
        setup::http::http_server(cli.bind_addr, cli.health_bind_addr, store).await?;

    // Wait for all servers to finish.
    tokio::select! {
        _ = http_server => log::info!("HTTP server shutdown."),
        _ = health_server => log::info!("Health server shutdown."),
    }

    Ok(())
}
