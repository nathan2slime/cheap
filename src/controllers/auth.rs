use crate::{
    database::entity::sessions,
    errors,
    services::auth::{
        dto::{CreateUser, SignIn},
        mutation::Mutation as auth_mutation,
    },
    types::{ContextData, ErrorData},
};
use actix_web::{
    post,
    web::{Data, Json},
    HttpResponse, Responder,
};
use tracing::error;

const TAG: &str = "Auth";

#[utoipa::path(
    post,
    tag = TAG,
    path = "/signup",
    request_body = CreateUser,
    responses(
        (status = CREATED, description = "Successfully", body = sessions::Model),
        (status = BAD_REQUEST, description = "Bad request", body = ErrorData),
        (status = CONFLICT, description = "Email already in use", body = ErrorData)
    )
)]
#[post("/signup")]
pub async fn signup(ctx: Data<ContextData>, body: Json<CreateUser>) -> impl Responder {
    match auth_mutation::signup(&ctx.db, &ctx.config, body.into_inner()).await {
        Ok(session) => HttpResponse::Created().json(session),
        Err(errors::SignUp::EmailInUse) => HttpResponse::Conflict().json(ErrorData {
            data: "Email already in use".to_string(),
        }),
        Err(errors::SignUp::InternalSignUp(e)) => {
            error!("{:?}", e);

            HttpResponse::BadRequest().json(ErrorData {
                data: format!("{:?}", e),
            })
        }
    }
}

#[utoipa::path(
    post,
    tag = TAG,
    path = "/signin",
    request_body = SignIn,
    responses(
        (status = CREATED, description = "Successfully", body = sessions::Model),
        (status = BAD_REQUEST, description = "Bad request", body = ErrorData),
        (status = UNAUTHORIZED, description = "Invalid credentials", body = ErrorData)
    )
)]
#[post("/signin")]
pub async fn signin(ctx: Data<ContextData>, body: Json<SignIn>) -> impl Responder {
    match auth_mutation::signin(&ctx.db, &ctx.config, body.into_inner()).await {
        Ok(session) => HttpResponse::Created().json(session),
        Err(errors::SignIn::InternalSignIn(e)) => HttpResponse::BadRequest().json(ErrorData {
            data: format!("{:?}", e),
        }),
        Err(_e) =>  HttpResponse::Unauthorized().json(ErrorData {
            data: "Invalid credentials".to_string(),
        })
    }
}
