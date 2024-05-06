use anyhow::Result;
use axum::Router;

use tracing::debug;

mod db;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt::init();

    let pool = db::init().await?;
    let app = Router::new();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

    debug!("Listening on {}...", listener.local_addr()?);
    axum::serve(listener, app).await?;

    Ok(())
}
