use actix_web::{App, HttpServer, Responder, web::Data};
use actix_web::dev::ServiceRequest;
use actix_web_openidconnect::ActixWebOpenId;

use state::App as AppState;

mod db;
mod routes;
mod state;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let (client_id, client_secret, redirect_url, issuer_url) = state::load_openid_configuration();
    let should_auth = |req: &ServiceRequest| {
        !req.path().starts_with("/no_auth") && req.method() != actix_web::http::Method::OPTIONS
    };
    let openid = ActixWebOpenId::init(
        client_id,
        client_secret,
        redirect_url,
        issuer_url,
        should_auth,
        None, // TODO: Check `Some("http://localhost:3000/is_auth/hello".to_string()),`.
        vec!["openid".to_string()],
    )
    .await;

    let state = AppState::new(db::init().await.expect("Failed to connect to database!"));
    HttpServer::new(move || {
        App::new()
            .wrap(openid.get_middleware())
            .configure(openid.configure_open_id())
            .app_data(Data::new(state.clone()))
            .service(routes::forum::create_forum)
            .service(routes::forum::get_forums)
            .service(routes::forum::get_forum)
            .service(routes::forum::update_forum)
            .service(routes::forum::delete_forum)
            .service(routes::message::send_message)
    })
    .bind(("0.0.0.0", 3000))?
    .run()
    .await
}
