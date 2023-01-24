use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum Error {
    BadRequest(String),
    InternalServerError(String),
    NotFound(String),
    Unauthorized(String),
    ValidateError(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            Error::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            Error::InternalServerError(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
            Error::NotFound(message) => (StatusCode::NOT_FOUND, message),
            Error::Unauthorized(message) => (StatusCode::UNAUTHORIZED, message),
            Error::ValidateError(message) => (StatusCode::BAD_REQUEST, message),
        };

        (status, error_message).into_response()
    }
}
impl From<diesel::result::Error> for Error {
    fn from(e: diesel::result::Error) -> Self {
        Self::InternalServerError(e.to_string())
    }
}

impl From<diesel_async::pooled_connection::bb8::RunError> for Error {
    fn from(value: diesel_async::pooled_connection::bb8::RunError) -> Self {
        Self::InternalServerError(value.to_string())
    }
}


