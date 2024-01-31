use std::sync::Arc;

use crate::backend::{
    main_route::AppState,
    models::{user::{User, RegisterUserSchema, LoginUserSchema}, tokenclaims::TokenClaims},
    utils::response_return_types::{CustomResponse, Error},
};
use argon2::{password_hash::SaltString, Argon2, PasswordHasher, PasswordHash, PasswordVerifier};
use axum::{extract::State, http::StatusCode, routing::{post, get}, Json, Router};
use axum_extra::extract::cookie::{Cookie,SameSite};
use cookie::CookieBuilder;
use jsonwebtoken::{encode, Header, EncodingKey};
use mongodb::bson::doc;
use rand_core::OsRng;
use serde_json::json;

pub fn router_auth() -> Router<Arc<AppState>> {
    Router::new()
    .route("/register", post(register_handler))
    .route("/login", post(login_user_handler))
    .route("/logout", get(logout_handler))
}

pub async fn register_handler(
    State(data_state): State<Arc<AppState>>,
    Json(body): Json<RegisterUserSchema>,
) -> Result<CustomResponse, Error> {
    let client = &data_state.client_db;
    let user_collection = client.database("Website").collection::<User>("Users");
    let filter = doc! { "$or": [ { "username": body.username.as_str() }, { "email": body.email.as_str() } ] };
    let user_exist = user_collection
        .find_one(
            filter,
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
        .map_err(|e| Error::ServerError(e.to_string()))?;

    let user_to_insert = User::new(None, body.username, body.email, hashing, 0);

    let user_insert_db = user_collection.insert_one(user_to_insert, None).await?;

    let id = user_insert_db.inserted_id.as_object_id().unwrap();
    let user_retrive_db = user_collection
        .find_one(doc! {"_id": id}, None)
        .await?
        .ok_or(Error::ServerError(String::from("Error in the retrive of the user in db")))?;
        
    
    let final_response = CustomResponse::new()
        .set_code(StatusCode::CREATED)
        .set_data(None);

    return Ok(final_response);
}


pub async fn login_user_handler(
    State(data_state): State<Arc<AppState>>,
    Json(body): Json<LoginUserSchema>,
) -> Result<CustomResponse, Error> {
    let client = &data_state.client_db;
    let user_collection = client.database("Website").collection::<User>("Users");
    let user_register: User = user_collection
        .find_one(
            doc! { "username": body.username.as_str()},
            None,
        )
        .await?
        .ok_or(Error::LoginError(String::from("User don't exist")))?; //need to be revisited

    let user_password = user_register.get_password();
    let user_hash_password = PasswordHash::new(&user_password)
        .map_err(|_error| Error::ServerError(String::from("Problem with user password hashing")))?;
    
    let is_valid = Argon2::default().verify_password(body.password.as_bytes(), &user_hash_password).map_or(false, |_| true);
    
    if !is_valid{
        return Err(Error::LoginError(String::from("User or password are wrong")));
    }

    let user_id = user_register.get_id();
    let now = chrono::Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + chrono::Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user_id,
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data_state.config_app.jwt_secret.as_ref()),
    )
    .map_err(|e| {
        return Error::ServerError(e.to_string())
    })?;


    let cookie = Cookie::build(("token",token))
        .path("/")
        .max_age(time::Duration::hours(1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .build();

    let final_response = CustomResponse::new()
        .set_code(StatusCode::OK)
        .set_header(axum::http::header::SET_COOKIE, cookie.to_string());
    Ok(final_response)
}


//testing logout
pub async fn logout_handler() -> Result<CustomResponse, Error> {
    let cookie = Cookie::build(("token", ""))
        .path("/")
        .max_age(time::Duration::hours(-1))
        .same_site(SameSite::Lax)
        .http_only(true)
        .build();

    //Needs a blacklist jwt thing... 
    
    let final_response = CustomResponse::new()
        .set_code(StatusCode::OK)
        .set_header(axum::http::header::SET_COOKIE, cookie.to_string());
    
    Ok(final_response)
}