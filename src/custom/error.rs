use rocket::{
    http::{ContentType, Status},
    request::Request,
    response::{self, Responder, Response},
    serde,
};
use serde::Serialize;
use std::io::Cursor;

#[derive(Serialize)]
pub struct ErrorResponse {
    error: Error,
}

#[derive(Serialize)]
pub struct Error {
    status: u16,
    message: String,
    cat: String,
}

#[derive(Debug, Clone)]
pub enum CustomError {
    NotFound(Status, String),
    ServiceUnavailable(Status, String),
    Default(Status),
}

impl CustomError {
    fn get_http_status(&self) -> Status {
        match self {
            CustomError::NotFound(status, _) => *status,
            CustomError::ServiceUnavailable(status, _) => *status,
            CustomError::Default(status) => *status,
        }
    }
}

impl std::fmt::Display for CustomError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(fmt, "Error {}.", self.get_http_status())
    }
}

impl<'r> Responder<'r, 'static> for CustomError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let err_response = match self.clone() {
            CustomError::Default(status) => serde::json::to_string(&ErrorResponse {
                error: Error {
                    status: status.code,
                    message: status.reason().unwrap().to_string(),
                    cat: format!("https://http.cat/{}.jpg", status.code),
                },
            })
            .unwrap(),
            CustomError::NotFound(status, details) => serde::json::to_string(&ErrorResponse {
                error: Error {
                    status: status.code,
                    message: format!(
                        "{} (No available {})",
                        status.reason().unwrap().to_string(),
                        details
                    ),
                    cat: format!("https://http.cat/{}.jpg", status.code),
                },
            })
            .unwrap(),
            CustomError::ServiceUnavailable(status, details) => {
                serde::json::to_string(&ErrorResponse {
                    error: Error {
                        status: status.code,
                        message: format!("{} ({})", status.reason().unwrap().to_string(), details),
                        cat: format!("https://http.cat/{}.jpg", status.code),
                    },
                })
                .unwrap()
            }
        };

        Response::build()
            .status(self.clone().get_http_status())
            .header(ContentType::JSON)
            .sized_body(err_response.len(), Cursor::new(err_response))
            .ok()
    }
}
