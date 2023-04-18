use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Login failed")]
    LoginFail,
    #[error("Authentication failure")]
    AuthFail,
    #[error("Ticket ID not found: {0}")]
    TicketIdNotFound(u64),
}

pub type Result<T = ()> = std::result::Result<T, Error>;

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR, "UNHANDLED_CLIENT_ERROR").into_response()
    }
}
