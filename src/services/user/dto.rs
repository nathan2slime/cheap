use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct CreateUser {
    pub email: String,
    pub username: String,
    pub password: String
}
