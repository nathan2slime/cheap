use actix_web::{
    post, web::{Data, Json}, HttpResponse, Responder
};
use sea_orm::DatabaseConnection;

use crate::{
    database::entity::users,
    services::user::{dto::CreateUser, mutation::Mutation},
};

#[utoipa::path(
    post,
    path = "/signup",
    request_body = CreateUser,
    responses(
        (status = 201, description = "Signup successfully", body = users::Model),
        (status = BAD_REQUEST, description = "BAD_REQUEST")
    )
)]
#[post("/signup")]
pub async fn signup(db: Data<DatabaseConnection>, body: Json<CreateUser>) -> impl Responder {
    let mut mutation = Mutation;
    let data = body.into_inner();

    let new_user = mutation.create(&db, data).await;

    match new_user {
        Ok(data) => HttpResponse::Created().json(data),
        Err(e) => HttpResponse::BadRequest().json(e),
    }
}
