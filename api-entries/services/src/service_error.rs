use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ServiceErrorType {
    #[error("BadInput. {0}")]
    BadInput(String),
    #[error("BadToken. {0}")]
    BadToken(String),
    #[error("OutdatedPayload. Payload was created too long time ago - create a new one with up to date timestamp")]
    OutdatedPayload,
    #[error("ResponseError. {0}")]
    ResponseError(String),
    #[error("Requested endpoint not found")]
    NotFound,
    #[error("IOError. {0}")]
    IOError(String),
}

pub trait HttpCodeForError {
    fn http_status_code(&self) -> u16;
}

impl HttpCodeForError for ServiceErrorType {
    fn http_status_code(&self) -> u16 {
        match self {
            ServiceErrorType::BadToken(_) => 400,
            ServiceErrorType::BadInput(_) => 400,
            ServiceErrorType::ResponseError(_) => 400,
            ServiceErrorType::IOError(_) => 500,
            ServiceErrorType::OutdatedPayload => 400,
            ServiceErrorType::NotFound => 404,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceError {
    pub error_code: u16,
    pub error: String,
}

impl ServiceError {
    pub fn new(service_error_type: ServiceErrorType) -> Self {
        Self {
            error_code: service_error_type.http_status_code(),
            error: service_error_type.to_string(),
        }
    }
}
