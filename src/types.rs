use sea_orm::{DatabaseConnection, DbErr};
use serde::Serialize;
use utoipa::{ToResponse, ToSchema};

use crate::config::AppConfig;

#[derive(Debug, Serialize, ToSchema, ToResponse)]
pub struct ErrorData {
    pub data: String,
}

#[derive(Debug, Clone)]
pub struct ContextData {
    pub db: DatabaseConnection,
    pub config: AppConfig,
}

#[derive(Debug)]
pub enum Error {
    EmailInUse,
    Internal(DbErr),
}
