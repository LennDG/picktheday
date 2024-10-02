pub type Result<T> = core::result::Result<T, Error>;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    // -- Operation Errors
    #[error("Entity was not found based on {0}")]
    EntityNotFound(String),
    // -- Database
    #[error("Database error: {0}")]
    DbErr(#[from] sea_orm::DbErr),
}
