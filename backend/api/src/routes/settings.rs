use actix_web::{delete, get, HttpResponse, post, put, Responder};
use actix_web::http::header::Header;
use actix_web::web::{Data, Json};

use crate::db::models::settings::Settings;
use crate::db::models::user::AccessLevel;
use crate::routes::APIError;
use crate::routes::user::{Credentials, has_access};
use crate::state::App;

#[get("/settings")]
pub async fn get_settings(app: Data<App>, credentials: Json<Credentials>) -> Result<impl Responder, APIError> {
    let has_access = has_access(app.config(), &credentials.user_id, &AccessLevel::Administrator).await.map_err(|_| APIError::InternalServerError)?;
    if !has_access {
        return Err(APIError::Unauthorized);
    }
    
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
    credentials: Json<Credentials>,
) -> Result<impl Responder, APIError> {
    let has_access = has_access(app.config(), &credentials.user_id, &AccessLevel::Administrator).await.map_err(|_| APIError::InternalServerError)?;
    if !has_access {
        return Err(APIError::Unauthorized);
    }
    
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
    credentials: Json<Credentials>,
) -> Result<impl Responder, APIError> {
    let has_access = has_access(app.config(), &credentials.user_id, &AccessLevel::Administrator).await.map_err(|_| APIError::InternalServerError)?;
    if !has_access {
        return Err(APIError::Unauthorized);
    }
    
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
    credentials: Json<Credentials>,
) -> Result<impl Responder, APIError> {
    let has_access = has_access(app.config(), &credentials.user_id, &AccessLevel::Administrator).await.map_err(|_| APIError::InternalServerError)?;
    if !has_access {
        return Err(APIError::Unauthorized);
    }
    
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
    credentials: Json<Credentials>,
) -> Result<impl Responder, APIError> {
    let has_access = has_access(app.config(), &credentials.user_id, &AccessLevel::Administrator).await.map_err(|_| APIError::InternalServerError)?;
    if !has_access {
        return Err(APIError::Unauthorized);
    }
    
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    let result = Settings::delete(&mut conn, &key)
        .await
        .map_err(|_| APIError::InternalServerError)?;
    
    Ok(HttpResponse::Ok().json(result))
}
