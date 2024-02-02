use crate::backend::main_route::AppState;
use axum::{routing::get, Router};
use std::sync::Arc;

use super::pages::index::render_index;

pub fn route_frontend() -> Router {
    Router::new().route("/", get(render_index))
}
