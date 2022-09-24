use std::io;
use thiserror::Error as THError;

// TODO: Move the errots to their own util library
#[allow(dead_code)]
#[derive(THError, Debug)]
pub enum ServiceIOError {
    #[error("The service returned empty")]
    Empty,
    #[error("The service crashed upon request.")]
    ServiceError,
    #[error("The service was not found.")]
    NotFoundError,
}

#[derive(THError, Debug)]
pub enum SamplingError {
    #[error("Was invalid input type.")]
    Disconnect(#[from] io::Error),
    #[error("Was not able to find the value for `{0}`.")]
    Redaction(String),
    #[error("Invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}

#[derive(THError, Debug)]
pub enum RouteSinkError {
    #[error("Was not able to find the value for `{0}`.")]
    Redaction(String),
    #[error("Invalid header (expected {expected:?}, found {found:?})")]
    InvalidHeader { expected: String, found: String },
    #[error("unknown data store error")]
    Unknown,
}

#[derive(THError, Debug)]
pub enum GatewayError {
    #[error("Unsupported Payload: {0:?} was not serializable")]
    PayloadNotSupported(String),
    #[error("Invalid header (expected {expected:?}, found {found:?})")]
    InvalidMetaOperation {
        expected: Vec<String>,
        found: String,
    },
    #[error("Invalid Meta Operation")]
    Unknown,
    #[error("Sink not found for response.")]
    SinkNotFound,
}
