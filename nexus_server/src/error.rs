use thiserror::Error;

#[derive(Error, Debug)]
pub enum NexusError {
    #[error("sea_orm::DbErr")]
    DbError(#[from] sea_orm::DbErr),

    #[error("invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}
