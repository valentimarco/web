use std::sync::Arc;

use axum::extract::State;
use axum::routing::{get, post};
use axum::{Router, extract::FromRef};
use mongodb::Client;

use crate::backend::config::{Config, self};

use super::database_connection::db_connection;
use super::routes::login_route::{register_handler, router_auth};

#[derive(Clone, FromRef)]
pub struct AppState{
    pub client_db: Client
}




pub async fn route_backend() -> Router {
    let config =  Config::init();
    
    
    // TODO: well well well, maybe i need to change this part xd 
    let app_state = Arc::new(AppState {
        client_db: db_connection(config.database_url).await.unwrap()
    });
    
    // https://stackoverflow.com/questions/40984932/what-happens-when-an-arc-is-cloned
    Router::new()
    .merge(router_auth().with_state(app_state.clone()))
    .with_state(app_state.clone())
    
    
}