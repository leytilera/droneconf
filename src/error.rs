use std::convert::Infallible;

use axum::{
    body::{Bytes, Empty},
    response::IntoResponse,
};
use hyper::StatusCode;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("File read error: {0}")]
    Read(#[from] std::io::Error),
    #[error("Deserialize error: {0}")]
    Deserialize(#[from] toml::de::Error),
    #[error("Axum error: {0}")]
    Axum(#[from] hyper::Error),
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),
    #[error("No content error")]
    NoContent,
}

impl IntoResponse for Error {
    type Body = Empty<Bytes>;

    type BodyError = Infallible;

    fn into_response(self) -> hyper::Response<Self::Body> {
        StatusCode::NO_CONTENT.into_response()
    }
}
