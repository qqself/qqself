use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServiceErrorType {
    #[error("BadInput. {0}")]
    BadInput(String),
    #[error("BadToken. {0}")]
    BadToken(String),
    #[error("OutdatedPayload. Payload was created too long time ago - create a new one with up to date timestamp")]
    OutdatedPayload,
    #[error("PaymentRequired. {0}")]
    PaymentRequired(String),
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
            ServiceErrorType::BadToken(_) => 498,
            ServiceErrorType::BadInput(_) => 422,
            ServiceErrorType::PaymentRequired(_) => 402,
            ServiceErrorType::IOError(_) => 502,
            ServiceErrorType::OutdatedPayload => 408,
            ServiceErrorType::NotFound => 404,
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceError {
    pub error_code: u16,
    pub error: String,
}
