use axum::{Router,response::IntoResponse};
use backend::models::user::UserSchema;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::backend::routes::login_route::__path_register_handler;




mod frontend;
mod backend;

#[derive(OpenApi)]
#[openapi(
    paths(register_handler),
    components(
        schemas(UserSchema)
    )
)]
struct ApiDoc;



async fn fallback(uri: axum::http::Uri) -> impl IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("No route {}", uri) )
}




//Semes like that merge fn don't accept a function that return the Router object, caused by askama_axum library

#[tokio::main]
async fn main(){
    env_logger::init();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let app = Router::new()
        .fallback(fallback)
        .merge(SwaggerUi::new("/swagger").url("/api-docs/openapi.json", ApiDoc::openapi()))
        .nest("/api/v1",crate::backend::main_route::route_backend().await )
        .nest("/", frontend::route::route_frontend());
    println!("Server open at: {}", "http://localhost:3000");
    axum::serve(listener,app).await.unwrap()
    
}