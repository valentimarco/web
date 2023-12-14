use axum::Json;
use axum::response::{IntoResponse,Response};
use axum::http::StatusCode;
use serde_json::{json, Value};

use log::debug;
use log::error;
use log::info;
use log::warn;

pub struct CustomResponse{
    status_code: StatusCode,
    status: String,
    data: Option<Json<Value>>
}

impl CustomResponse{
    pub fn new() -> Self{
        Self { status_code: StatusCode::OK, status: "success".to_string(),  data: None }
    }
    pub fn set_code(mut self,code: StatusCode) -> Self{
        self.status_code = code;
        self
    }
    
    pub fn set_status(mut self, status: &str) -> Self{
        self.status = status.to_string();
        self
    }

    pub fn set_data(mut self, data: Option<Json<Value>>) -> Self{
        self.data = data;
        self
    }
}

impl IntoResponse for CustomResponse{
    fn into_response(self) -> Response {
        let mut response = json!({
            "status": self.status,
            "data" : null
        });
        if let Some(Json(json_response_from_handler)) = self.data{
            response["data"] = json_response_from_handler
            
        }
        let json_response = Json(response);
        (
            self.status_code,
            json_response
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

    pub fn set_code(mut self,code: StatusCode) -> Self{
        self.status_code = code;
        self
    }
    
    pub fn set_message(mut self, message: &str) -> Self{
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
    MongoError(mongodb::error::Error),
    RegisterError(),
    GenericError(String)

    
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
                ErrorResponse::new().set_code(StatusCode::INTERNAL_SERVER_ERROR).set_message("Database error")
            },
            Self::RegisterError() =>{
                error!("Error Register");
                ErrorResponse::new().set_code(StatusCode::CONFLICT).set_message("user already register with this email and username")
            }
            Self::GenericError(message) => {
                error!("{}", message);
                ErrorResponse::new().set_message(message.as_str())
            }
            

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