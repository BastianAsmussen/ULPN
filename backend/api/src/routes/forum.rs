use actix_web::{delete, get, HttpResponse, post, put, Responder};
use actix_web::web::{Data, Json, Path, Query};
use serde::Deserialize;

use crate::db::models::forum::{Forum, NewForum};
use crate::db::models::user::AccessLevel;
use crate::routes::APIError;
use crate::routes::user::{Credentials, has_access};
use crate::state::App;

#[post("/forum")]
pub async fn create_forum(
    app: Data<App>,
    forum: Json<NewForum>,
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
    
    let forum = forum
        .into_inner()
        .insert(&mut conn)
        .await
        .map_err(|_| APIError::InternalServerError)?;

    Ok(HttpResponse::Created().json(forum))
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub limit: Option<i64>,
    pub access_level: Option<AccessLevel>,
    pub is_locked: Option<bool>,
}

#[get("/forum")]
pub async fn get_forums(
    app: Data<App>,
    info: Query<Info>,
    credentials: Json<Option<Credentials>>,
) -> Result<impl Responder, APIError> {
    let info = info.into_inner();

    let limit = info.limit.unwrap_or(10);
    let access_level = info.access_level.unwrap_or(AccessLevel::Child);
    let is_locked = info.is_locked.unwrap_or(true);
    tracing::info!("fetched info!");

    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    tracing::info!("connected to db!");

    let mut forums = Forum::all(&mut conn, limit, access_level, is_locked)
        .await
        .map_err(|_| APIError::InternalServerError)?;
    tracing::info!("connected to db!");

    let mut filtered_forums = Vec::new();
    for forum in &forums {
        let has_access = match credentials.0 {
            Some(ref credentials) => has_access(app.config(), &credentials.user_id, &forum.access_level).await.map_err(|_| APIError::InternalServerError)?,
            None => forum.access_level == AccessLevel::Child,
        };
        tracing::info!("filtered forum!");

        if !has_access {
            continue
        }

        filtered_forums.push(forum);
    }

    Ok(HttpResponse::Ok().json(forums))
}

#[get("/forum/{forum_id}")]
pub async fn get_forum(
    app: Data<App>,
    forum_id: Path<i32>,
    credentials: Json<Credentials>,
) -> Result<impl Responder, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    let forum = Forum::by_id(&mut conn, forum_id.into_inner())
        .await
        .map_err(|_| APIError::InternalServerError)?;

    let has_access = has_access(app.config(), &credentials.user_id, &forum.access_level).await.map_err(|_| APIError::InternalServerError)?;
    if !has_access {
        return Err(APIError::Unauthorized);
    }

    Ok(HttpResponse::Ok().json(forum))
}

#[put("/forum/{forum_id}")]
pub async fn update_forum(
    app: Data<App>,
    forum_id: Path<i32>,
    forum: Json<NewForum>,
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

    Forum::delete(&mut conn, forum_id.into_inner())
        .await
        .map_err(|_| APIError::InternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}
