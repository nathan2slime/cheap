use sea_orm::{Database, DatabaseConnection};
use tracing::{error, info};

pub async fn create(db_url: &str) -> Option<DatabaseConnection> {
    let connection = Database::connect(db_url).await;

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
