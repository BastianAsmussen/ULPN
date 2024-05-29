use actix_web::{get, HttpResponse, Responder};
use actix_web_openidconnect::openid_middleware::Authenticated;

// Implementation details: https://viden.stil.dk/display/OFFSKOLELOGIN/Implementering+af+tjeneste#tab-OIDC+-+Login+endpoint+og+parametre
#[get("/auth/login")]
pub async fn login(auth_data: Authenticated) -> impl Responder {
    
}

#[get("/auth/redirect")]
pub async fn redirect(auth_data: Authenticated) -> impl Responder {
    
}

#[get("/auth/logout")]
pub async fn logout(auth_data: Authenticated) -> impl Responder {
    
}
