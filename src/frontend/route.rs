use axum::{Router, routing::{Route, get}};

use super::pages::index::render_index;




pub fn route_frontend() -> Router {
    Router::new()
        .route("/", get(render_index))

}
