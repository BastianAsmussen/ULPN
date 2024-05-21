use actix_web::{web::Data, App, HttpServer};
use anyhow::Result;

use state::App as AppState;

mod db;
mod routes;
mod state;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    tracing_subscriber::fmt::init();

    let state = AppState::new(db::init().await?);
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await?;

    Ok(())
}

