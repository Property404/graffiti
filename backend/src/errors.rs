use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Request too big")]
    RequestTooBig,
}

pub type Result<T = (), E = Error> = std::result::Result<T, E>;

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
