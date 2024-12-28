use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
       let _add_password_col = manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .add_column(ColumnDef::new(Users::Password).string().not_null())
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
       let _drop_password_col =  manager
            .alter_table(
                Table::alter()
                    .table(Users::Table)
                    .drop_column(Users::Password)
                    .to_owned(),
            )
            .await;

        manager
            .drop_table(Table::drop().table(Sessions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Sessions {
    Table,
    Id,
    AuthToken,
    RefreshToken,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Password,
}
