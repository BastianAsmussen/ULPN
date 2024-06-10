use crate::db::models::forum::{Forum, NewForum};
use crate::db::models::user::AccessLevel;
use actix_web::web::{Data, Json, Path, Query};
use actix_web::{delete, get, post, put, HttpResponse, Responder};
use serde::Deserialize;

use crate::routes::user::AuthenticationGuard;
use crate::routes::APIError;
use crate::state::App;

#[post("/forum")]
pub async fn create_forum(
    app: Data<App>,
    forum: Json<NewForum>,
    authentication_guard: AuthenticationGuard,
) -> Result<impl Responder, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;

    if !authentication_guard
        .has_access(&mut conn, &AccessLevel::Administrator)
        .await
    {
        return Err(APIError::Unauthorized);
    }

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
    pub access_level: Option<AccessLevel>,
    pub is_locked: Option<bool>,
}

#[get("/forum")]
pub async fn get_forums(
    app: Data<App>,
    info: Query<Info>,
    authentication_guard: AuthenticationGuard,
) -> Result<impl Responder, APIError> {
    let info = info.into_inner();

    let limit = info.limit.unwrap_or(10);
    let access_level = info.access_level.unwrap_or(AccessLevel::Child);
    let is_locked = info.is_locked.unwrap_or(true);

    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;

    if !authentication_guard
        .has_access(&mut conn, &access_level)
        .await
    {
        return Err(APIError::Unauthorized);
    }

    let forums = Forum::all(&mut conn, limit, access_level, is_locked)
        .await
        .map_err(|_| APIError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(forums))
}

#[get("/forum/{forum_id}")]
pub async fn get_forum(
    app: Data<App>,
    forum_id: Path<i32>,
    authentication_guard: AuthenticationGuard,
) -> Result<impl Responder, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    let forum = Forum::by_id(&mut conn, forum_id.into_inner())
        .await
        .map_err(|_| APIError::InternalServerError)?;

    if !authentication_guard
        .has_access(&mut conn, &forum.access_level)
        .await
    {
        return Err(APIError::Unauthorized);
    }

    Ok(HttpResponse::Ok().json(forum))
}

#[put("/forum/{forum_id}")]
pub async fn update_forum(
    app: Data<App>,
    forum_id: Path<i32>,
    forum: Json<NewForum>,
    authentication_guard: AuthenticationGuard,
) -> Result<impl Responder, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    if !authentication_guard
        .has_access(&mut conn, &AccessLevel::Administrator)
        .await
    {
        return Err(APIError::Unauthorized);
    }

    let forum = forum
        .into_inner()
        .update(&mut conn, forum_id.into_inner())
        .await
        .map_err(|_| APIError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(forum))
}

#[delete("/forum/{forum_id}")]
pub async fn delete_forum(
    app: Data<App>,
    forum_id: Path<i32>,
    authentication_guard: AuthenticationGuard,
) -> Result<impl Responder, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    if !authentication_guard
        .has_access(&mut conn, &AccessLevel::Administrator)
        .await
    {
        return Err(APIError::Unauthorized);
    }

    Forum::delete(&mut conn, forum_id.into_inner())
        .await
        .map_err(|_| APIError::InternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}
