pub mod model;

pub mod error;

pub type Result<T> = std::result::Result<T, error::NexusError>;
