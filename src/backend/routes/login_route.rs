use std::sync::Arc;

use crate::backend::{
    main_route::AppState,
    models::user::{User, UserSchema},
    utils::response_return_types::{CustomResponse, Error},
};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
use askama_axum::IntoResponse;
use axum::{debug_handler, extract::State, http::StatusCode, routing::post, Json, Router};
use mongodb::bson::doc;
use rand_core::OsRng;
use serde_json::json;

pub fn router_auth() -> Router<Arc<AppState>> {
    Router::new().route("/register", post(register_handler))
}

#[debug_handler]
pub async fn register_handler(
    State(data_state): State<Arc<AppState>>,
    Json(body): Json<UserSchema>,
) -> Result<CustomResponse, Error> {
    let client = &data_state.client_db;
    let user_collection = client.database("Website").collection::<User>("Users");
    let user_exist = user_collection
        .find_one(
            doc! { "username": body.username.as_str(),"email": body.email.as_str() },
            None,
        )
        .await?;

    if let Some(_) = user_exist {
        let response = Error::RegisterError();
        return Err(response);
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashing = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|error| Error::GenericError(error.to_string()))?;

    let user_to_insert = User::new(None, body.username, body.email, hashing, 0);

    let user_insert_db = user_collection.insert_one(user_to_insert, None).await?;

    let id = user_insert_db.inserted_id.as_object_id().unwrap();
    let user_retrive_db = user_collection
        .find_one(doc! {"_id": id}, None)
        .await?
        .ok_or(Error::GenericError(String::from(
            "Error in the insertion of the user",
        )))?;

    let json_respone = json!({
        "username": user_retrive_db.get_username()
    });
    let final_response = CustomResponse::new()
        .set_code(StatusCode::CREATED)
        .set_data(Some(Json(json_respone)));

    return Ok(final_response);
}
