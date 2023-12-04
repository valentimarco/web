use std::sync::Arc;

use crate::backend::{
    main_route::AppState,
    models::user::{self, UserSchema, User},
    Result,
};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use axum::{debug_handler, extract::State, http::StatusCode, routing::post, Json, Router};
use mongodb::bson::{bson, doc, Bson};
use rand_core::OsRng;
use serde_json::{json, Value};



pub fn router_auth() -> Router<Arc<AppState>> {
    Router::new().route("/register", post(register_handler))
}


#[debug_handler]
pub async fn register_handler(State(data_state): State<Arc<AppState>>,Json(body): Json<UserSchema>,) -> Result<Json<Value>> {
    let client = &data_state.client_db;
    let user_collection = client.database("Website").collection::<User>("Users");
    let user_exist = user_collection
        .find_one(
            doc! { "username": body.username.as_str(),"email": body.email.as_str() },
            None,
        )
        .await
        .unwrap();
    
    if let Some(exist) = user_exist{
        todo!()
    }


    

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map_err(|e| {
            todo!()
        })
        .map(|hash| hash.to_string())?;

    let user_to_insert = User::new(None, body.username, body.email, hashed_password, 0);
    
    let user_insert_db = user_collection
    .insert_one(user_to_insert, None).await.unwrap();
    let id = user_insert_db.inserted_id.as_object_id().unwrap();
    let user_retrive_db = user_collection.find_one(doc! {"_id": id}, None).await.unwrap();

    if let None = user_retrive_db{
        todo!()
    }

    Ok(Json(json!({"status": "success", "data": "todo: return the object from db"})))
}
