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
    #[error("Invalid uri: {0}")]
    UriInvalid(#[from] http::uri::InvalidUri),

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

        // Create a placeholder Axum reponse.
        let mut response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        // Insert the Error into the reponse.
        response.extensions_mut().insert(Arc::new(self));

        response
    }
}
// endregion: --- Axum IntoResponse
