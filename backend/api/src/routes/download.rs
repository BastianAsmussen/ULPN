use actix_web::{get, HttpResponse, Responder};

#[get("/download")]
pub async fn download_app() -> impl Responder {
    let page = r#"
        <h1>Klik <a href="https://github.com/BastianAsmussen/ULPN/releases/download/v0.1.0/ulpn.apk">her</a> for at downloade appen!</h1>
    "#;
    
    HttpResponse::Ok().body(page)
}
