use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RegisterUserDTO {
    pub username: String,
    pub email: String,
    pub password: String,
}
