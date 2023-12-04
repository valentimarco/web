use axum::response::{IntoResponse,Response};
use axum::http::StatusCode;

#[derive(Debug)]
pub enum Error{
    LoginFail,
    AuthFail
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode,ClientError) {
        match self {
            Self::LoginFail => (StatusCode::FORBIDDEN, ClientError::LoginFail),
            Self::AuthFail => (StatusCode::FORBIDDEN, ClientError::NoAuth),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, ClientError::ServiceError)

        }
    }
    
}

impl IntoResponse for Error{
    fn into_response(self) -> Response{
        println!("{:<12} - {self:?}","Into-res");

        (StatusCode::IM_A_TEAPOT, "wa").into_response()
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for Error {}

#[derive(Debug,strum_macros::AsRefStr)]
pub enum ClientError{
    LoginFail,
    NoAuth,
    InvalidParmas,
    ServiceError
}