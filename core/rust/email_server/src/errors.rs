use actix_web::{error::ResponseError, HttpResponse};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum APIError {
    #[display(fmt = "Internal Server Error occured")]
    InternalServerError,

    #[display(fmt = "IO Error occured")]
    IOError,

    #[display(fmt = "Error during parsing process")]
    ParseError
}

impl ResponseError for APIError {
    fn error_response(&self) -> HttpResponse {
        match self {
            APIError::InternalServerError => HttpResponse::InternalServerError().finish(),
            APIError::IOError => HttpResponse::Conflict().body(format!("{}", &self)),
            APIError::ParseError => HttpResponse::Conflict().body(format!("{}", &self)),
        }
    }
}

impl From<std::io::Error> for APIError {
    fn from(_: std::io::Error) -> Self {
          APIError::IOError
    }
}

impl From<serde_json::error::Error> for APIError {
    fn from(_: serde_json::error::Error) -> Self {
        APIError::ParseError
    } 
}