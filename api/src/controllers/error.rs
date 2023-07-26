use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};

use crate::domains::error::Error;

impl ResponseError for Error {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match *self {
            Error::Conflicts => StatusCode::CONFLICT,
            Error::DbError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
