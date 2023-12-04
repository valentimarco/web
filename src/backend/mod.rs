pub mod routes;
pub mod config;
pub mod database_connection;
pub mod models;
pub mod middlewares;
pub mod error;
pub mod main_route;


pub type Result<T> = std::result::Result<T,crate::backend::error::Error>;