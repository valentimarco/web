use std::sync::Arc;

use crate::backend::{
    main_route::AppState,
    models::user::{self, UserSchema, User}, utils::response_return_types::{ErrorResponse, Error}
};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use askama_axum::IntoResponse;
use axum::{debug_handler, extract::State, http::StatusCode, routing::post, Json, Router};
use mongodb::bson::{bson, doc, Bson};
use rand_core::OsRng;
use serde_json::{json, Value};



pub fn router_auth() -> Router<Arc<AppState>> {
    Router::new().route("/register", post(register_handler))
}


#[debug_handler]
pub async fn register_handler(State(data_state): State<Arc<AppState>>,Json(body): Json<UserSchema>,) -> impl IntoResponse {
    let client = &data_state.client_db;
    let user_collection = client.database("Website").collection::<User>("Users");
    let user_exist = user_collection
        .find_one(
            doc! { "username": body.username.as_str(),"email": body.email.as_str() },
            None,
        )
        .await
        .unwrap();
    
    if let Some(exist) = user_exist {
        return (StatusCode::CONFLICT, Json(json!({"error": "user already register with this email and username"})))
                .into_response();
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashing = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map(|hash| hash.to_string());

    let hashed_password = match hashing {
        Err(e) =>{
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
                .into_response();
        },
        Ok(hash) => hash
    };

    
    let user_to_insert = User::new(None, body.username, body.email, hashed_password, 0);
    
    let user_insert_db = user_collection
        .insert_one(user_to_insert, None)
        .await;

    match user_insert_db{
        Err(e) =>{
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
                .into_response();
        },
        Ok(user_insert) =>{
            let id = user_insert.inserted_id.as_object_id();
            match id{
                None => {
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "There was a problem with the database: User inserted wasn't found"})))
                        .into_response();
                },
                Some(id) => {
                    let user_retrive_db = user_collection
                        .find_one(doc! {"_id": id}, None)
                        .await;
                    match user_retrive_db{
                        Err(e) => {
                            return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
                                .into_response();
                        },
                        Ok(option_user) => {
                            match option_user {
                                None => {
                                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "There was a problem with the database: User inserted wasn't found"})))
                                        .into_response();
                                },
                                Some(user) =>{
                                    return (StatusCode::CREATED, Json(user))
                                        .into_response();
                                }
                            }
                        }

                    }
                }
            }
        }
    }


    
}
