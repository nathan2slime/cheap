use sea_orm::{ColumnTrait, DbConn, EntityTrait, QueryFilter, Set};

use super::dto::CreateUser;
use crate::database::entity::users;
use crate::types::Error;
use tracing::error;

pub struct Mutation;

impl Mutation {
    pub async fn create(
        &mut self,
        db: &DbConn,
        form_data: CreateUser,
    ) -> Result<users::Model, Error> {
        let email_is_in_use = self.find_by_email(db, form_data.email.clone()).await;

        if email_is_in_use.is_some() {
            error!("email already in use");

            Err(Error {
                data: String::from("email already in use"),
            })
        } else {
            let data = users::ActiveModel {
                id: Set(uuid::Uuid::new_v4()),
                email: Set(form_data.email.to_owned()),
                username: Set(form_data.username.to_owned()),
                ..Default::default()
            };

            let opts = users::Entity::insert(data).exec(db).await.unwrap();
            let id = opts.last_insert_id;

            let user = users::Entity::find()
                .filter(users::Column::Id.eq(id))
                .one(db)
                .await
                .unwrap();

            match user {
                Some(user) => Ok(user),
                None => Err(Error {
                    data: String::from("user not created"),
                }),
            }
        }
    }
    pub async fn find_by_email(&self, db: &DbConn, email: String) -> Option<users::Model> {
        match users::Entity::find()
            .filter(users::Column::Email.eq(email))
            .one(db)
            .await
        {
            Ok(user) => user,
            Err(_e) => {
                error!("{:?}", _e);

                None
            }
        }
    }
}
