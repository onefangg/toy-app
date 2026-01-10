use std::fmt::{Debug, Display, Formatter};
use axum::http::StatusCode;
use axum::response::IntoResponse;

#[derive(Debug)]
pub struct ErrorResponse {
    pub status_code: StatusCode,
    pub message: String,
}


pub fn internal_error<E>(err: E) -> ErrorResponse
where
    E: std::error::Error,
{
    ErrorResponse {status_code: StatusCode::INTERNAL_SERVER_ERROR , message:err.to_string() }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("Auth error occurred with status code {:?} and {:?}", self.status_code, self.message);
        self.status_code.into_response()
    }
}


#[derive(Debug)]
pub struct AuthError {
    pub status_code: StatusCode,
    pub message: &'static str,
}

impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("Auth error occurred with status code {:?} and {:?}", self.status_code, self.message);
        self.status_code.into_response()
    }
}


#[derive(Debug)]
pub enum UsernameError {
    TooShort
}
impl Display for UsernameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            UsernameError::TooShort => { f.write_str("Username is too short")}
        }
    }
}
pub enum PasswordError {
    TooShort
}

impl Debug for PasswordError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("Redacted value :)")
    }
}

impl Display for PasswordError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PasswordError::TooShort => { f.write_str("Password is too short")}
        }
    }
}