use sea_orm::{ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter, Set};

use crate::{database::entity::users, services::auth::dto::CreateUser};
use bcrypt::{hash, DEFAULT_COST};
use tracing::error;

pub struct Mutation;

impl Mutation {
    pub async fn create(db: &DbConn, form_data: CreateUser) -> Result<users::Model, DbErr> {
        let password = hash(form_data.password.to_owned(), DEFAULT_COST).unwrap();

        let data = users::ActiveModel {
            id: Set(uuid::Uuid::new_v4()),
            email: Set(form_data.email.to_owned()),
            username: Set(form_data.username.to_owned()),
            password: Set(password),
            ..Default::default()
        };

        users::Entity::insert(data).exec_with_returning(db).await
    }
    pub async fn find_by_email(db: &DbConn, email: String) -> Option<users::Model> {
        match users::Entity::find()
            .filter(users::Column::Email.eq(email))
            .one(db)
            .await
        {
            Ok(user) => user,
            Err(error) => {
                error!("{:?}", error);

                None
            }
        }
    }
}
