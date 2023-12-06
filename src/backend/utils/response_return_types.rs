use axum::Json;
use axum::response::{IntoResponse,Response};
use axum::http::StatusCode;
use serde::Serialize;
use serde_json::{json, Value};


struct CustomResponse{
    status_code: StatusCode,
    data: Json<Value>
}

impl CustomResponse{
    pub fn new(code: StatusCode, json: Json<Value>) -> Self{
        Self { status_code: code,  data: json }
    }
}

impl IntoResponse for CustomResponse{
    fn into_response(self) -> Response {
        (
            self.status_code, 
            self.data
        ).into_response()
    }
}


pub struct ErrorResponse{
    status_code: StatusCode,
    message: String
}

impl ErrorResponse {
    pub fn new(code: StatusCode, message: &str) -> Self{
        Self { status_code: code, message: message.to_string() }
    }
}

impl IntoResponse for ErrorResponse{
    fn into_response(self) -> Response {
        (
            self.status_code, 
            Json(json!({ "message": self.message}))
        ).into_response()
        
    }
}


#[derive(Debug)]
pub enum Error{
    LoginFail(String),
    ServerError(String)
    
}

impl Error {
    pub fn client_status_and_error(&self) -> ErrorResponse{
        match self {
            // Self::LoginFail(message) => ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, &message),
            Self::ServerError(message) => ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, &message),
            _ => ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, "Server Exploded"),
            

        }
    }
    
}

impl IntoResponse for Error{
    fn into_response(self) -> Response{
        self
        .client_status_and_error()
        .into_response()

        
    }
}

// impl std::fmt::Display for Error {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{}", self)
//     }
// }

// impl std::error::Error for Error {}

// #[derive(Debug,strum_macros::AsRefStr)]
// pub enum ClientError{
//     LoginFail,
//     NoAuth,
//     InvalidParmas,
//     ServiceError
// }