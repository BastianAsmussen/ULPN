use actix_session::Session;
use actix_web::get;
use actix_web::web::Data;
use crate::state::App;

#[get("/login")]
pub async fn login(session: Session, data: Data<App>) -> impl actix_web::Responder {
    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();
    
    todo!()
}
