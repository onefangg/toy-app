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
    ErrorResponse {status_code: StatusCode::INTERNAL_SERVER_ERROR , message: err.to_string() }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("Auth error occurred with status code {:?} and {:?}", self.status_code, self.message);
        self.status_code.into_response()
    }
}