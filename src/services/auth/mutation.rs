use crate::{config::AppConfig, services::user::dto::CreateUser};
use crate::{
    database::entity::sessions,
    services::{
        session::mutation::Mutation as session_mutation, user::mutation::Mutation as user_mutation,
    },
    types::Error,
};
use sea_orm::DatabaseConnection;

pub struct Mutation;

impl Mutation {
    pub async fn signup(
        db: &DatabaseConnection,
        config: &AppConfig,
        data: CreateUser,
    ) -> Result<sessions::Model, Error> {
        if user_mutation::find_by_email(db, data.email.clone())
            .await
            .is_some()
        {
            return Err(Error::EmailInUse);
        }

        let user = user_mutation::create(db, data)
            .await
            .map_err(Error::Internal)?;

        session_mutation::create(config, db, user)
            .await
            .map_err(Error::Internal)
    }
}
