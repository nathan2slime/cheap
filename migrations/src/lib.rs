pub use sea_orm_migration::prelude::*;

mod m20220101_000001_init;
mod m20241227_184847_add_auth;
mod m20241228_230547_relation_in_session;
mod m20241228_232247_email_not_null;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_init::Migration),
            Box::new(m20241227_184847_add_auth::Migration),
            Box::new(m20241228_230547_relation_in_session::Migration),
            Box::new(m20241228_232247_email_not_null::Migration),
        ]
    }
}
