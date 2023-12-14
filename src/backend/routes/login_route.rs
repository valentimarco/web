use std::sync::Arc;

use crate::backend::{
    main_route::AppState,
    models::user::{User, UserSchema},
    utils::response_return_types::Error
};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use askama_axum::IntoResponse;
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use mongodb::bson::doc;
use rand_core::OsRng;
use serde_json::json;






pub fn router_auth() -> Router<Arc<AppState>> {
    Router::new().route("/register", post(register_handler))
        
}

#[utoipa::path(post, path="/api/v1/register", request_body= UserSchema, responses((status=201, description="User created successfully")))]
pub async fn register_handler(
    State(data_state): State<Arc<AppState>>,
    Json(body): Json<UserSchema>,
) -> Result<impl IntoResponse, Error> {
    let client = &data_state.client_db;
    let user_collection = client.database("Website").collection::<User>("Users");
    let user_exist = user_collection
        .find_one(
            doc! { "username": body.username.as_str(),"email": body.email.as_str() },
            None,
        )
        .await?;

    if let Some(exist) = user_exist {
        let response = (
            StatusCode::CONFLICT,
            Json(json!({"error": "user already register with this email and username"})),
        )
        .into_response();
        return Ok(response);
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashing = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map(|hash| hash.to_string());

    let hashed_password = match hashing {
        Err(e) => {
            let response = (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
                .into_response();
            return Ok(response);
        }
        Ok(hash) => hash,
    };

    let user_to_insert = User::new(None, body.username, body.email, hashed_password, 0);

    let user_insert_db = user_collection.insert_one(user_to_insert, None).await?;

    let id = user_insert_db.inserted_id.as_object_id().unwrap();
    let user_retrive_db = user_collection.find_one(doc! {"_id": id}, None).await?.unwrap();
    
    return Ok((StatusCode::CREATED, Json(user_retrive_db)).into_response());
}
