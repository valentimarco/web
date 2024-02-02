use std::sync::Arc;

use axum::body::Body;
use axum::http::Response;
use axum::{
    extract::State,
    http::{header, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
    Json,
};

use axum_extra::extract::cookie::CookieJar;
use jsonwebtoken::{decode, DecodingKey, Validation};


use mongodb::bson::doc;
use serde::Serialize;

use crate::backend::main_route::AppState;
use crate::backend::models::token_claims::TokenClaims;
use crate::backend::models::user::User;
use crate::backend::utils::response_return_types::{CustomResponse, Error};

pub async fn auth(
    cookie_jar: CookieJar,
    State(data): State<Arc<AppState>>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, Error> {
    let token = cookie_jar
        .get("token")
        .map(|cookie| cookie.value().to_string())
        .or_else(|| {
            req.headers()
                .get(header::AUTHORIZATION)
                .and_then(|auth_header| auth_header.to_str().ok())
                .and_then(|auth_value| {
                    if auth_value.starts_with("Bearer ") {
                        Some(auth_value[7..].to_owned())
                    } else {
                        None
                    }
                })
        })
        .ok_or(Error::AuthError(String::from("No auth cookie or header")))?;

    let claims = decode::<TokenClaims>(
        &token,
        &DecodingKey::from_secret(data.config_app.jwt_secret.as_ref()),
        &Validation::default(),
    )
    .map_err(|_| Error::LoginError(String::from("Invalid Token")))?
    .claims;
    let client = &data.client_db;
    let user_collection = client.database("Website").collection::<User>("Users");
    println!("{:?}", claims.sub);

    let user_id = mongodb::bson::oid::ObjectId::parse_str(&claims.sub).map_err(|_| Error::AuthError(String::from("Invalid user id")))?; // convert the user id to a bson object id
    let user = user_collection
        .find_one(doc! {"_id": user_id }, None)
        .await?
        .ok_or(Error::AuthError(String::from("User don't exist")))?;

    // store the user so other middlewares or handlers can access it
    req.extensions_mut().insert(user);
    Ok(next.run(req).await)
}
