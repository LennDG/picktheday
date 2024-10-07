use std::sync::Arc;

use axum::response::{IntoResponse, Response};
use http::StatusCode;
use thiserror::Error;
use tracing::debug;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    // -- Validation failure
    #[error("Invalid plan: {0}")]
    NewPlanInvalid(String),
    #[error("Invalid user: {0}")]
    NewUserInvalid(String),
    #[error("Invalid uri: {0}")]
    UriInvalid(#[from] http::uri::InvalidUri),

    // -- Entity
    #[error("Entity not found: {0}")]
    EntityNotFound(#[from] entity::error::Error),

    // -- External
    #[error("Database error: {0}")]
    DbErr(#[from] entity::sea_orm::DbErr),
    #[error("Dotenvy error: {0}")]
    Dotenvy(#[from] dotenvy::Error),
    #[error("std env error: {0}")]
    StdEnv(#[from] std::env::VarError),
}

// region:    --- Axum IntoResponse
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        debug!("{:<12} - model::Error {self:?}", "INTO_RES");

        // TODO! Create a mapping from entity not found to 404 etc.

        // Create a placeholder Axum reponse.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the reponse.
        response.extensions_mut().insert(Arc::new(self));

        response
    }
}
// endregion: --- Axum IntoResponse
