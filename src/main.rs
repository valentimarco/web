use axum::{Router,response::IntoResponse};




mod frontend;
mod backend;


async fn fallback(uri: axum::http::Uri) -> impl IntoResponse {
    (axum::http::StatusCode::NOT_FOUND, format!("No route {}", uri) )
}




//Semes like that merge fn don't accept a function that return the Router object, caused by askama_axum library

#[tokio::main]
async fn main(){
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    let app = Router::new()
        .fallback(fallback)
        .nest("/api/v1",crate::backend::main_route::route_backend().await )
        .nest("/", frontend::route::route_frontend());
    println!("Server open at: {}", "http://localhost:3000");
    axum::serve(listener,app).await.unwrap()
    
}