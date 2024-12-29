use crate::config::AppConfig;
use crate::errors;
use crate::{
    database::entity::sessions,
    services::{
        session::mutation::Mutation as session_mutation, user::mutation::Mutation as user_mutation,
    },
};
use bcrypt::verify;
use sea_orm::DatabaseConnection;
use tracing::error;

use super::dto::{CreateUser, SignIn};

pub struct Mutation;

impl Mutation {
    pub async fn signup(
        db: &DatabaseConnection,
        config: &AppConfig,
        data: CreateUser,
    ) -> Result<sessions::Model, errors::SignUp> {
        if user_mutation::find_by_email(db, data.email.clone())
            .await
            .is_some()
        {
            return Err(errors::SignUp::EmailInUse);
        }

        let user = user_mutation::create(db, data)
            .await
            .map_err(errors::SignUp::InternalSignUp)?;

        session_mutation::create(config, db, user)
            .await
            .map_err(errors::SignUp::InternalSignUp)
    }

    pub async fn signin(
        db: &DatabaseConnection,
        config: &AppConfig,
        data: SignIn,
    ) -> Result<sessions::Model, errors::SignIn> {
        match user_mutation::find_by_email(db, data.email).await {
            Some(user) => {
                let match_password = verify(data.password, &user.password);

                match match_password {
                    Ok(true) => session_mutation::create(config, db, user)
                        .await
                        .map_err(errors::SignIn::InternalSignIn),
                    Ok(false) => Err(errors::SignIn::InvalidCredentials),
                    Err(e) => {
                        error!("{:?}", e);

                        Err(errors::SignIn::InvalidCredentials)
                    }
                }
            }
            None => Err(errors::SignIn::InvalidCredentials),
        }
    }
}
