use crate::{
    services::auth::mutation::Mutation as auth_mutation,
    types::{ContextData, Error, ErrorData},
};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use tracing::error;

use crate::{database::entity::users, services::user::dto::CreateUser};

const TAG: &str = "Auth";

#[utoipa::path(
    post,
    tag = TAG,
    path = "/signup",
    request_body = CreateUser,
    responses(
        (status = CREATED, description = "Successfully", body = users::Model),
        (status = BAD_REQUEST, description = "Bad request", body = ErrorData),
        (status = CONFLICT, description = "Email already in use", body = ErrorData)
    )
)]
#[post("/signup")]
pub async fn signup(ctx: Data<ContextData>, body: Json<CreateUser>) -> impl Responder {
    match auth_mutation::signup(&ctx.db, &ctx.config, body.into_inner()).await {
        Ok(session) => HttpResponse::Created().json(session),
        Err(Error::EmailInUse) => HttpResponse::Conflict().json(ErrorData {
            data: "Email already in use".to_string(),
        }),
        Err(Error::Internal(e)) => {
            error!("{:?}", e);
            HttpResponse::BadRequest().json(ErrorData {
                data: format!("{:?}", e),
            })
        }
    }
}
