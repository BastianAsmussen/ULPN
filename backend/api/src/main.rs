use actix_web::{web::Data, App, HttpServer};

use state::App as AppState;

mod db;
mod routes;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let state = AppState::new(db::init().await.expect("Failed to connect to database!"));
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(state.clone()))
            .service(routes::forum::create_forum)
            .service(routes::forum::get_forums)
            .service(routes::forum::get_forum)
            .service(routes::forum::update_forum)
            .service(routes::forum::delete_forum)
            .service(routes::message::send_message)
            .service(routes::message::get_message)
            .service(routes::message::update_message)
            .service(routes::message::delete_message)
            .service(routes::settings::get_settings)
            .service(routes::settings::get_setting)
            .service(routes::settings::create_setting)
            .service(routes::settings::update_setting)
            .service(routes::settings::delete_setting)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
