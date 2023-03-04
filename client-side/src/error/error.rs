use std::fmt::{Display, Formatter};

use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};


#[derive(Debug)]
pub enum ServerErrorResponse {
    InternalServerError(ErrorResponse),
    BadReq(ErrorResponse),
    NotFound(ErrorResponse),
    Unauthorized(ErrorResponse),
}

impl ResponseError for ServerErrorResponse {
    fn error_response(&self) -> HttpResponse {
        match self {
            ServerErrorResponse::InternalServerError(errors) => HttpResponse::Ok().json(errors),
            ServerErrorResponse::BadReq(errors) => HttpResponse::Ok().json(errors),
            ServerErrorResponse::NotFound(errors) => HttpResponse::Ok().json(errors),
            ServerErrorResponse::Unauthorized(errors) => HttpResponse::Ok().json(errors),
        }
    }
}

#[derive(Debug)]
pub enum Error {
    BadRequest(String),
    InternalServerError(String),
    NotFound(String),
    UnauthorizedError(String),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ErrorResponse {
    pub code: i64,
    pub message: String,
    pub data: Option<i64>,   // i64 here for filling the data but it's always none.
}


impl From<ErrorResponse> for ServerErrorResponse {
    fn from(error_response: ErrorResponse) -> Self {
        Self::InternalServerError(error_response)
    }
}


impl From<Error> for ServerErrorResponse {
    fn from(error: Error) -> Self {
        match error {
            Error::BadRequest(error_msg) => Self::from(ErrorResponse { code: 400, message: error_msg, data: None }),
            Error::InternalServerError(error_msg) => Self::from(ErrorResponse { code: 500, message: error_msg, data: None }),
            Error::NotFound(error_msg) => Self::from(ErrorResponse { code: 404, message: error_msg, data: None }),
            Error::UnauthorizedError(message) => Self::from(ErrorResponse { code: 401, message, data: None }),
        }
    }
}

impl Display for ServerErrorResponse {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ServerErrorResponse::InternalServerError(_) => write!(f, "Internal Server Error Display."),
            ServerErrorResponse::BadReq(_) => write!(f, "Bas Request Display."),
            ServerErrorResponse::NotFound(_) => write!(f, "Not Found Display."),
            ServerErrorResponse::Unauthorized(_) => write!(f, "Unauthorized Display."),
        }
    }
}