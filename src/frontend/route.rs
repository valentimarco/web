use axum::{routing::{get, post}, Router};


use super::pages::{count::render_count, index::render_index};

pub fn route_frontend() -> Router {
    Router::new()
    .route("/", get(render_index))
    .route("/count", post(render_count))
}
