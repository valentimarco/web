use axum::Json;
use axum::response::{IntoResponse,Response};
use axum::http::StatusCode;
use serde_json::{json, Value};

use log::debug;
use log::error;
use log::info;
use log::warn;

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
    pub fn new() -> Self{
        Self { status_code: StatusCode::INTERNAL_SERVER_ERROR, message: String::from("Server Exploded") }
    }

    pub fn code(mut self,code: StatusCode) -> Self{
        self.status_code = code;
        self
    }
    
    pub fn message(mut self, message: &str) -> Self{
        self.message = message.to_string();
        self
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
    MongoError(mongodb::error::Error)
    
}

impl From<mongodb::error::Error> for Error {
    fn from(value: mongodb::error::Error) -> Self {
        Error::MongoError(value)
    }
}

impl Error {
    pub fn client_status_and_error(&self) -> ErrorResponse{
        match self {
            // Self::LoginFail(message) => ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, &message),
            // Self::ServerError(message) => ErrorResponse::new(StatusCode::INTERNAL_SERVER_ERROR, &message),
            Self::MongoError(error) => {
                let error_db = error;
                error!("Error Database {}", error_db);
                ErrorResponse::new().code(StatusCode::INTERNAL_SERVER_ERROR).message("Database error")
            },
            _ => ErrorResponse::new(),
            

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