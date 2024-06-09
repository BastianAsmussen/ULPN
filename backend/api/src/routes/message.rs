use actix_web::web::{Data, Json};
use actix_web::{post, HttpResponse, Responder};

use crate::db::models::message::NewMessage;
use crate::routes::APIError;
use crate::state::App;

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
