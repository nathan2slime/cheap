use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _user_ = manager
            .create_table(
                Table::create()
                    .table(Users::Table)
                    .if_not_exists()
                    .col(pk_uuid(Users::Id))
                    .col(string(Users::Username))
                    .col(string(Users::Password))
                    .col(ColumnDef::new(Users::Email).string().unique_key())
                    .to_owned(),
            )
            .await;

        manager
            .create_table(
                Table::create()
                    .table(Sessions::Table)
                    .if_not_exists()
                    .col(pk_uuid(Sessions::Id))
                    .col(string(Sessions::AuthToken))
                    .col(string(Sessions::RefreshToken))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let _user_ = manager
            .drop_table(Table::drop().table(Users::Table).to_owned())
            .await;

        manager
            .drop_table(Table::drop().table(Sessions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
    Username,
    Password,
    Email,
}

#[derive(DeriveIden)]
enum Sessions {
    Table,
    Id,
    AuthToken,
    RefreshToken,
}
