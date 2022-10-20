pub mod routes;

use actix_web::{http::header::ContentType, HttpResponse};
use qqself_core::datetime::Timestamp;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug)]
pub enum HttpErrorType {
    BadInput(String),
    BadToken(String),
    OutdatedPayload,
    PaymentRequired(String),
    AccountErr(String),
    IOError(String),
}

impl std::error::Error for HttpErrorType {}

#[derive(Debug, Serialize, Deserialize)]
pub struct HttpError {
    pub timestamp: Timestamp,
    pub error_code: u16,
    pub error: String,
}

impl Display for HttpErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            HttpErrorType::BadToken(s) => format!("BadToken. {s}"),
            HttpErrorType::BadInput(s) => format!("BadInput. {s}"),
            HttpErrorType::PaymentRequired(s) => format!("PaymentRequired. {s}"),
            HttpErrorType::IOError(s) => format!("IOError. {s}"),
            HttpErrorType::OutdatedPayload => "OutdatedPayload. Payload was created too long time ago - create a new one with up to date timestamp, check /info endpoint for server timestamp".to_string(),
            HttpErrorType::AccountErr(s) => format!("AccountErr. {s}"),
        };
        f.write_str(&s)
    }
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
