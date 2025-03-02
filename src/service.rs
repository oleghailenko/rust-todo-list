use std::fmt::{Display, Formatter};
use std::io::Cursor;
use rocket::http::Status;
use rocket::{Request, Response};
use rocket::response::Responder;
use serde_json::json;
use crate::service::Error::InternalError;

pub mod user;

#[derive(Debug)]
pub enum Error {
    AlreadyExists,
    InternalError,
}

#[derive(Debug)]
pub struct AppError {
    pub error: Error,
    pub message: String,
}

impl Display for AppError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl From<sqlx::Error> for AppError {
    fn from(value: sqlx::Error) -> Self {
        Self {
            error: InternalError,
            message: value.to_string(),
        }
    }
}

impl std::error::Error for AppError {}

impl<'r, 'o: 'r> Responder<'r, 'o> for AppError {
    fn respond_to(self, _request: &'r Request<'_>) -> rocket::response::Result<'o> {
        let status = match self.error {
            Error::AlreadyExists => Status::Conflict,
            _ => Status::InternalServerError,
        };

        let body= json!({ "message": self.message }).to_string();

        Response::build()
            .raw_header("Content-Type", "application/json")
            .status(status)
            .sized_body(body.len(), Cursor::new(body))
            .ok()
    }
}