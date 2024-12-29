use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Sessions::Table)
                    .add_column(uuid(Sessions::UserId).not_null())
                    .add_foreign_key(
                        TableForeignKey::new()
                            .name("fk_user_session")
                            .from_tbl(Sessions::Table)
                            .from_col(Sessions::UserId)
                            .to_tbl(Users::Table)
                            .to_col(Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Sessions::Table)
                    .drop_foreign_key(Alias::new("fk_user_session"))
                    .drop_column(Sessions::UserId)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Sessions {
    Table,
    UserId,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
