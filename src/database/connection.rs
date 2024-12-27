use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use tracing::{error, info};

pub async fn create(db_url: &str) -> Option<DatabaseConnection> {
    let mut opts = ConnectOptions::new(db_url);
    opts.set_schema_search_path("default");

    let connection = Database::connect(opts).await;

    match connection {
        Ok(connection) => {
            info!(
                "conected in database with pool size {}",
                connection.get_postgres_connection_pool().size()
            );

            Some(connection)
        }
        Err(error) => {
            error!("{:?}", error);

            None
        }
    }
}
