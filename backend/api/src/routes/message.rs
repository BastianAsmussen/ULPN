use actix_web::web::{Data, Json, Path, Query};
use actix_web::{post, HttpResponse, Responder, get, delete};
use serde::Deserialize;
use crate::db::models::forum::Forum;

use crate::db::models::message::{Message, NewMessage};
use crate::db::models::user::AccessLevel;
use crate::routes::APIError;
use crate::routes::user::AuthenticationGuard;
use crate::state::App;

#[derive(Deserialize)]
pub struct Info {
    pub limit: Option<i64>,
}

#[post("/message")]
pub async fn send_message(
    app: Data<App>,
    message: Json<NewMessage>,
    authentication_guard: AuthenticationGuard,
) -> Result<impl Responder, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    
    let forum = Forum::by_id(&mut conn, message.forum_id)
        .await
        .map_err(|_| APIError::InternalServerError)?;
    if !authentication_guard.has_access(&mut conn, &forum.access_level).await {
        return Err(APIError::Unauthorized);
    }
    
    let message = message
        .into_inner()
        .insert(&mut conn)
        .await
        .map_err(|_| APIError::InternalServerError)?;

    Ok(HttpResponse::Created().json(message))
}

#[get("/message/{message_id}")]
pub async fn get_message(app: Data<App>, message_id: Path<i64>, authentication_guard: AuthenticationGuard) -> Result<impl Responder, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    let message = Message::by_id(&mut conn, message_id.into_inner())
        .await
        .map_err(|_| APIError::InternalServerError)?;
    
    let forum = Forum::by_id(&mut conn, message.forum_id).await.map_err(|_| APIError::InternalServerError)?;
    if !authentication_guard.has_access(&mut conn, &forum.access_level).await {
        return Err(APIError::Unauthorized);
    }

    Ok(HttpResponse::Ok().json(message))
}

#[post("/message/{message_id}")]
pub async fn update_message(
    app: Data<App>,
    message_id: Path<i64>,
    new_message: Json<NewMessage>,
    authentication_guard: AuthenticationGuard,
) -> Result<impl Responder, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    
    let message = Message::by_id(&mut conn, message_id.into_inner())
        .await
        .map_err(|_| APIError::InternalServerError)?;
    
    let forum = Forum::by_id(&mut conn, message.forum_id).await.map_err(|_| APIError::InternalServerError)?;
    if !authentication_guard.has_access(&mut conn, &forum.access_level).await {
        return Err(APIError::Unauthorized);
    }
    
    if message.sender_id != authentication_guard.user_id.parse::<i32>().map_err(|_| APIError::Unauthorized)? {
        return Err(APIError::Unauthorized);
    }
    
    let new_message = new_message.into_inner();
    let message = message
        .update(&mut conn, new_message)
        .await
        .map_err(|_| APIError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(message))
}

#[delete("/message/{message_id}")]
pub async fn delete_message(app: Data<App>, message_id: Path<i64>, authentication_guard: AuthenticationGuard) -> Result<impl Responder, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    
    let message =
        Message::by_id(&mut conn, message_id.into_inner())
        .await
        .map_err(|_| APIError::InternalServerError)?;
    
    let is_admin = authentication_guard.has_access(&mut conn, &AccessLevel::Administrator).await;
    if is_admin || message.sender_id != authentication_guard.user_id.parse::<i32>().map_err(|_| APIError::Unauthorized)? {
        return Err(APIError::Unauthorized);
    }
    
    message
        .delete(&mut conn)
        .await
        .map_err(|_| APIError::InternalServerError)?;
    
    Ok(HttpResponse::NoContent().finish())
}
