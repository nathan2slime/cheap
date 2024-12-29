use sea_orm::DatabaseConnection;
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
