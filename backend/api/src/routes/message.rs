use actix_web::web::{Data, Json, Path, Query};
use actix_web::{post, HttpResponse, Responder, get, delete};
use serde::Deserialize;

use crate::db::models::message::{Message, NewMessage};
use crate::routes::APIError;
use crate::state::App;

#[derive(Deserialize)]
pub struct Info {
    pub limit: Option<i64>,
}

#[post("/message")]
pub async fn send_message(
    app: Data<App>,
    message: Json<NewMessage>,
) -> Result<impl Responder, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    let message = message
        .into_inner()
        .insert(&mut conn)
        .await
        .map_err(|_| APIError::InternalServerError)?;

    Ok(HttpResponse::Created().json(message))
}

#[get("/message")]
pub async fn get_messages(app: Data<App>, info: Query<Info>) -> Result<impl Responder, APIError> {
    let info = info.into_inner();
    
    let limit = info.limit.unwrap_or(10);
    
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    let messages = Message::all(&mut conn, limit).await.map_err(|_| APIError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(messages))
}

#[get("/message/{message_id}")]
pub async fn get_message(app: Data<App>, message_id: Path<i64>) -> Result<impl Responder, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    let message = Message::by_id(&mut conn, message_id.into_inner())
        .await
        .map_err(|_| APIError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(message))
}

#[post("/message/{message_id}")]
pub async fn update_message(
    app: Data<App>,
    message_id: Path<i64>,
    new_message: Json<NewMessage>,
) -> Result<impl Responder, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    
    let message = Message::by_id(&mut conn, message_id.into_inner())
        .await
        .map_err(|_| APIError::InternalServerError)?;
    
    let new_message = new_message.into_inner();
    let message = message
        .update(&mut conn, new_message)
        .await
        .map_err(|_| APIError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(message))
}

#[delete("/message/{message_id}")]
pub async fn delete_message(app: Data<App>, message_id: Path<i64>) -> Result<impl Responder, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    
    Message::by_id(&mut conn, message_id.into_inner())
        .await
        .map_err(|_| APIError::InternalServerError)?
        .delete(&mut conn)
        .await
        .map_err(|_| APIError::InternalServerError)?;
    
    Ok(HttpResponse::NoContent().finish())
}