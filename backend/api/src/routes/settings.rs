use actix_web::{delete, get, HttpResponse, post, put, Responder};
use actix_web::web::{Data, Json};
use crate::db::models::settings::Settings;
use crate::routes::APIError;
use crate::state::App;

#[get("/settings")]
pub async fn get_settings(app: Data<App>) -> Result<impl Responder, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    let settings = Settings::all(&mut conn)
        .await
        .map_err(|_| APIError::InternalServerError)?;
    
    Ok(HttpResponse::Ok().json(settings))
}

#[get("/settings/{key}")]
pub async fn get_setting(
    app: Data<App>,
    key: String,
) -> Result<impl Responder, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    let setting = Settings::by_key(&mut conn, &key)
        .await
        .map_err(|_| APIError::InternalServerError)?;
    
    Ok(HttpResponse::Ok().json(setting))
}

#[post("/settings")]
pub async fn create_setting(
    app: Data<App>,
    setting: Json<Settings>,
) -> Result<impl Responder, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    let result = Settings::insert(&mut conn, &setting.key, &setting.value)
        .await
        .map_err(|_| APIError::InternalServerError)?;
    
    Ok(HttpResponse::Created().json(result))
}

#[put("/settings/{key}")]
pub async fn update_setting(
    app: Data<App>,
    key: String,
    setting: Json<Settings>,
) -> Result<impl Responder, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    let result = Settings::update(&mut conn, &key, &setting.value)
        .await
        .map_err(|_| APIError::InternalServerError)?;
    
    Ok(HttpResponse::Ok().json(result))
}

#[delete("/settings/{key}")]
pub async fn delete_setting(
    app: Data<App>,
    key: String,
) -> Result<impl Responder, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    let result = Settings::delete(&mut conn, &key)
        .await
        .map_err(|_| APIError::InternalServerError)?;
    
    Ok(HttpResponse::Ok().json(result))
}
