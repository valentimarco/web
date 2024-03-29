use std::sync::Arc;

use axum::{extract::FromRef, Router};
use mongodb::Client;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};

use super::routes::login_route::router_auth;
use super::utils::config_app::ConfigApp;
use super::utils::database_connection::db_connection;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub client_db: Client,
    pub config_app: ConfigApp,
}

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::backend::routes::login_route::register_handler,
        crate::backend::routes::login_route::login_user_handler
    ),
    components(
        schemas(crate::backend::models::dto::register_user_dto::RegisterUserDTO),
        schemas(crate::backend::models::dto::login_user_dto::LoginUserDTO),
    ),
    servers(
        (url="/api/v1")
    ),
    modifiers(&SecurityAddon),
    tags(
        (name = "todo", description = "Todo items management API")
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "api_key",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("todo_apikey"))),
            )
        }
    }
}

pub async fn route_backend() -> Router {
    let config = ConfigApp::init();

    // TODO: well well well, maybe i need to change this part xd
    let app_state = Arc::new(AppState {
        client_db: db_connection(config.database_url.clone()).await.unwrap(),
        config_app: config,
    });

    // https://stackoverflow.com/questions/40984932/what-happens-when-an-arc-is-cloned
    Router::new()
        .merge(router_auth(app_state.clone()))
        .with_state(app_state)
}
