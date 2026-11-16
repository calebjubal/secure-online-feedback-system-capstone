#![forbid(unsafe_code)]

use std::{error::Error, net::SocketAddr};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();
    let address: SocketAddr = std::env::var("BIND_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:3000".into())
        .parse()?;
    let listener = tokio::net::TcpListener::bind(address).await?;
    tracing::info!(address = %listener.local_addr()?, "API listening");
    axum::serve(listener, feedback_backend::app())
        .with_graceful_shutdown(async {
            if let Err(error) = tokio::signal::ctrl_c().await {
                tracing::error!(%error, "Failed to listen for shutdown signal");
            }
        })
        .await?;
    Ok(())
}
