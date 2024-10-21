use std::sync::Arc;

use sea_orm::DatabaseConnection;

pub mod db;
pub mod entity;
pub mod error;

pub type Result<T> = std::result::Result<T, error::NexusError>;
pub type DbPool = Arc<DatabaseConnection>;