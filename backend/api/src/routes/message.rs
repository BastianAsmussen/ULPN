use actix_web::{delete, get, HttpResponse, post, put, Responder};
use actix_web::web::{Data, Json, Path};
use serde::Deserialize;
use crate::db::models::forum::Forum;

use crate::db::models::message::{Message, NewMessage};
use crate::routes::APIError;
use crate::routes::APIError::InternalServerError;
use crate::routes::user::{Credentials, has_access};
use crate::state::App;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub limit: Option<i64>,
}

#[post("/message")]
pub async fn send_message(
    app: Data<App>,
    message: Json<NewMessage>,
    credentials: Json<Credentials>,
) -> Result<impl Responder, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    
    let forum = Forum::by_id(&mut conn, message.forum_id).await.map_err(|_| InternalServerError)?;
    let has_access = has_access(app.config(), &credentials.user_id, &forum.access_level).await.map_err(|_| APIError::InternalServerError)?;
    if !has_access {
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
pub async fn get_message(
    app: Data<App>,
    message_id: Path<i64>,
    credentials: Json<Credentials>,
) -> Result<impl Responder, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    let message = Message::by_id(&mut conn, message_id.into_inner())
        .await
        .map_err(|_| APIError::InternalServerError)?;

    let forum = Forum::by_id(&mut conn, message.forum_id).await.map_err(|_| InternalServerError)?;
    let has_access = has_access(app.config(), &credentials.user_id, &forum.access_level).await.map_err(|_| APIError::InternalServerError)?;
    if !has_access {
        return Err(APIError::Unauthorized);
    }

    Ok(HttpResponse::Ok().json(message))
}

#[put("/message/{message_id}")]
pub async fn update_message(
    app: Data<App>,
    message_id: Path<i64>,
    new_message: Json<NewMessage>,
    credentials: Json<Credentials>,
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
pub async fn delete_message(
    app: Data<App>,
    message_id: Path<i64>,
) -> Result<impl Responder, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;

    let message = Message::by_id(&mut conn, message_id.into_inner())
        .await
        .map_err(|_| APIError::InternalServerError)?;
    
    message
        .delete(&mut conn)
        .await
        .map_err(|_| APIError::InternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}
