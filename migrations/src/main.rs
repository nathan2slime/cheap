use sea_orm_migration::prelude::*;

#[warn(unused_must_use)]
#[async_std::main]
async fn main() {
    cli::run_cli(migrations::Migrator).await;
}
