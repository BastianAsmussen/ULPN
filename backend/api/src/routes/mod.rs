use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

pub mod forum;
pub mod user;
mod message;

#[derive(Debug, Error)]
pub enum APIError {
    #[error("Internal Server Error")]
    InternalServerError,
}

impl ResponseError for APIError {
    fn status_code(&self) -> StatusCode {
        match *self {
            Self::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).finish()
    }
}
