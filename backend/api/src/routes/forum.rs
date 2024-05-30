use crate::db::models::forum::{Forum, NewForum};
use actix_web::web::{Data, Json, Path, Query};
use actix_web::{delete, get, post, HttpResponse, Responder};
use actix_web_openidconnect::openid_middleware::Authenticated;
use serde::Deserialize;

use crate::routes::APIError;
use crate::state::App;

#[post("/no-auth/forum")]
pub async fn create_forum(
    app: Data<App>,
    auth: Authenticated,
    forum: Json<NewForum>,
) -> Result<impl Responder, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    let forum = forum
        .into_inner()
        .insert(&mut conn)
        .await
        .map_err(|_| APIError::InternalServerError)?;

    Ok(HttpResponse::Created().json(forum))
}

#[derive(Deserialize)]
pub struct Info {
    pub limit: Option<i64>,
}

#[get("/no-auth/forum")]
pub async fn get_forums(app: Data<App>, info: Query<Info>) -> Result<impl Responder, APIError> {
    let limit = info.limit.unwrap_or(10);

    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    let forums = Forum::all(&mut conn, limit)
        .await
        .map_err(|_| APIError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(forums))
}

#[get("/no-auth/forum/{forum_id}")]
pub async fn get_forum(app: Data<App>, forum_id: Path<i32>) -> Result<impl Responder, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    let forum = Forum::by_id(&mut conn, forum_id.into_inner())
        .await
        .map_err(|_| APIError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(forum))
}

#[post("/no-auth/forum/{forum_id}")]
pub async fn update_forum(
    app: Data<App>,
    forum_id: Path<i32>,
    forum: Json<NewForum>,
) -> Result<impl Responder, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    let forum = forum
        .into_inner()
        .update(&mut conn, forum_id.into_inner())
        .await
        .map_err(|_| APIError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(forum))
}

#[delete("/no-auth/forum/{forum_id}")]
pub async fn delete_forum(app: Data<App>, forum_id: Path<i32>) -> Result<impl Responder, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    Forum::delete(&mut conn, forum_id.into_inner())
        .await
        .map_err(|_| APIError::InternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}
