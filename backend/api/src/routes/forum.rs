use crate::db::models::forum::{Forum, NewForum};
use actix_web::web::{Data, Json, Path};
use actix_web::{delete, get, post, HttpResponse};

use crate::routes::APIError;
use crate::state::App;

#[post("/forum")]
pub async fn create_forum(app: Data<App>, forum: Json<NewForum>) -> Result<HttpResponse, APIError> {
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

#[get("/forum/{forum_id}")]
pub async fn get_forum(app: Data<App>, forum_id: Path<i32>) -> Result<HttpResponse, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    let forum = Forum::by_id(&mut conn, forum_id.into_inner())
        .await
        .map_err(|_| APIError::InternalServerError)?;

    Ok(HttpResponse::Ok().json(forum))
}

#[post("/forum/{forum_id}")]
pub async fn update_forum(
    app: Data<App>,
    forum_id: Path<i32>,
    forum: Json<NewForum>,
) -> Result<HttpResponse, APIError> {
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
pub async fn delete_forum(app: Data<App>, forum_id: Path<i32>) -> Result<HttpResponse, APIError> {
    let mut conn = app
        .establish_connection()
        .await
        .map_err(|_| APIError::InternalServerError)?;
    Forum::delete(&mut conn, forum_id.into_inner())
        .await
        .map_err(|_| APIError::InternalServerError)?;

    Ok(HttpResponse::NoContent().finish())
}
