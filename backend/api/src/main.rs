use actix_web::{App, HttpServer, web::Data};

use state::App as AppState;

use anyhow::Result;

mod db;
mod routes;
mod state;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt::init();

    let state = AppState::new(db::init().await?);
    HttpServer::new(move || App::new().app_data(Data::new(state.clone())))
        .bind(("0.0.0.0", 3000))?
        .run()
        .await?;

    Ok(())
}
