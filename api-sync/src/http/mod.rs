pub mod routes;

use actix_web::{http::header::ContentType, HttpResponse};
use qqself_core::datetime::Timestamp;
use serde::{Deserialize, Serialize};

use thiserror::Error;

#[derive(Error, Debug)]
pub enum HttpErrorType {
    #[error("BadInput. {0}")]
    BadInput(String),
    #[error("BadToken. {0}")]
    BadToken(String),
    #[error("OutdatedPayload. Payload was created too long time ago - create a new one with up to date timestamp, check /info endpoint for server timestamp")]
    OutdatedPayload,
    #[error("PaymentRequired. {0}")]
    PaymentRequired(String),
    #[error("AccountErr. {0}")]
    AccountErr(String),
    #[error("IOError. {0}")]
    IOError(String),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpError {
    pub timestamp: Timestamp,
    pub error_code: u16,
    pub error: String,
}

impl actix_web::error::ResponseError for HttpErrorType {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::from_u16(match self {
            HttpErrorType::BadToken(_) => 498,
            HttpErrorType::BadInput(_) => 422,
            HttpErrorType::PaymentRequired(_) => 402,
            HttpErrorType::IOError(_) => 502,
            HttpErrorType::OutdatedPayload => 408,
            HttpErrorType::AccountErr(_) => 502,
        })
        .unwrap()
    }

    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let body = HttpError {
            timestamp: Timestamp::now(),
            error_code: self.status_code().as_u16(),
            error: self.to_string(),
        };
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(body)
    }
}
