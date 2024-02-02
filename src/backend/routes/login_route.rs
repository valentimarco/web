use std::sync::Arc;

use crate::backend::middlewares::mw_auth_jwt::auth;
use crate::backend::models::dto::login_user_dto::LoginUserDTO;
use crate::backend::models::dto::register_user_dto::RegisterUserDTO;
use crate::backend::{
    main_route::AppState,
    models::{token_claims::TokenClaims, user::User},
    utils::response_return_types::{CustomResponse, Error},
};
use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use axum::middleware::from_fn_with_state;
use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use axum_extra::extract::cookie::{Cookie, SameSite};
use jsonwebtoken::{encode, EncodingKey, Header};
use mongodb::bson::doc;
use rand_core::OsRng;

pub fn router_auth(app_state: Arc<AppState>) -> Router<Arc<AppState>> {
    Router::new()
        .route("/register", post(register_handler))
        .route("/login", post(login_user_handler))
        .route(
            "/logout",
            get(logout_handler).route_layer(from_fn_with_state(app_state.clone(), auth)),
        )
}

#[utoipa::path(
    post,
    path = "/register",
    responses(
        (status = 201, body = [RegisterUserDTO]),
        (status = 500),
        (status = 409, description="User already exists")
    )
)]
pub async fn register_handler(
    State(data_state): State<Arc<AppState>>,
    Json(body): Json<RegisterUserDTO>,
) -> Result<CustomResponse, Error> {
    let client = &data_state.client_db;
    let user_collection = client.database("Website").collection::<User>("Users");
    let filter = doc! { "$or": [ { "username": body.username.as_str() }, { "email": body.email.as_str() } ] };
    let user_exist = user_collection.find_one(filter, None).await?;

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
    user_collection
        .find_one(doc! {"_id": id}, None)
        .await?
        .ok_or(Error::ServerError(String::from(
            "Error in the retrive of the user in db",
        )))?;
    //TODO: The registration must create an jwt token

    let final_response = CustomResponse::new()
        .set_code(StatusCode::CREATED)
        .set_data(None);

    return Ok(final_response);
}
#[utoipa::path(
    post,
    path = "/login",
    responses(
        (status = 201, body = [LoginUserDTO]),
        (status = 400)
    )
)]
pub async fn login_user_handler(
    State(data_state): State<Arc<AppState>>,
    Json(body): Json<LoginUserDTO>,
) -> Result<CustomResponse, Error> {
    let client = &data_state.client_db;
    let user_collection = client.database("Website").collection::<User>("Users");
    let user_register: User = user_collection
        .find_one(doc! { "username": body.username.as_str()}, None)
        .await?
        .ok_or(Error::LoginError(String::from("User don't exist")))?; //need to be revisited

    let user_password = user_register.get_password();
    let user_hash_password = PasswordHash::new(&user_password)
        .map_err(|_error| Error::ServerError(String::from("Problem with user password hashing")))?;

    let is_valid = Argon2::default()
        .verify_password(body.password.as_bytes(), &user_hash_password)
        .map_or(false, |_| true);

    if !is_valid {
        return Err(Error::LoginError(String::from(
            "User or password are wrong",
        )));
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
    .map_err(|e| return Error::ServerError(e.to_string()))?;

    let cookie = Cookie::build(("token", token))
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
