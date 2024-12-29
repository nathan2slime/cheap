use std::str::FromStr;

use sea_orm::{DatabaseConnection, DbErr, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    config::AppConfig,
    database::entity::{sessions, users},
    services::jwt,
};

pub struct Mutation;

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct JwtPayload {
    sub: String,
    iat: i64,
    exp: i64,
}

impl Mutation {
    pub async fn create(
        config: &AppConfig,
        db: &DatabaseConnection,
        user: users::Model,
    ) -> Result<sessions::Model, DbErr> {
        let session_secret = config.session_secret.to_owned();
        let user_id = user.id.to_owned().to_string();

        let auth_token =
            jwt::mutation::Mutation::create(session_secret.clone(), user_id.clone(), 30)
                .await
                .unwrap();

        let refresh_token =
            jwt::mutation::Mutation::create(session_secret.clone(), user_id.clone(), 30)
                .await
                .unwrap();
            
        let data = sessions::ActiveModel {
            id: Set(uuid::Uuid::new_v4()),
            auth_token: Set(auth_token),
            refresh_token: Set(refresh_token),
            user_id: Set(Uuid::from_str(&user_id).unwrap()),
            ..Default::default()
        };

        sessions::Entity::insert(data).exec_with_returning(db).await
    }
}
