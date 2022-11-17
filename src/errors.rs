use actix_web::{
    body::BoxBody, http::StatusCode, HttpRequest, HttpResponse, Responder, ResponseError,
};

use crate::schemas::errors::ErrorSchema;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("{0}")]
    InternalServer(String),
    #[error("{0}")]
    BAdRequest(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Forbidden(String),
}

pub trait TodoError {
    type Output;

    fn server_err(self, message: &str) -> Self::Output;
    fn bad_request_err(self, message: &str) -> Self::Output;
    fn not_found_err(self, message: &str) -> Self::Output;
    fn forbidden_err(self, message: &str) -> Self::Output;

    fn database_err(self) -> Self::Output;
    fn already_username_err(self, username: &str) -> Self::Output;
    fn key_creation_err(self) -> Self::Output;
    fn invalid_token_err(self) -> Self::Output;
    fn user_not_found_err(self) -> Self::Output;
}

impl<T, E> TodoError for std::result::Result<T, E> {
    type Output = Result<T>;

    fn server_err(self, message: &str) -> Self::Output {
        self.map_err(|_| Error::InternalServer(message.to_string()))
    }

    fn bad_request_err(self, message: &str) -> Self::Output {
        self.map_err(|_| Error::BAdRequest(message.to_string()))
    }

    fn not_found_err(self, message: &str) -> Self::Output {
        self.map_err(|_| Error::NotFound(message.to_string()))
    }

    fn forbidden_err(self, message: &str) -> Self::Output {
        self.map_err(|_| Error::Forbidden(message.to_string()))
    }

    fn database_err(self) -> Self::Output {
        self.server_err("Database error ):")
    }

    fn already_username_err(self, username: &str) -> Self::Output {
        self.bad_request_err(&format!("Username {} already exists", username))
    }

    fn key_creation_err(self) -> Self::Output {
        self.server_err("Error while creating the key ):")
    }

    fn invalid_token_err(self) -> Self::Output {
        self.forbidden_err("The token is invalid")
    }

    fn user_not_found_err(self) -> Self::Output {
        self.bad_request_err("User not found")
    }
}

impl<T> TodoError for Option<T> {
    type Output = Result<T>;

    fn server_err(self, message: &str) -> Self::Output {
        self.ok_or("/:").server_err(message)
    }

    fn bad_request_err(self, message: &str) -> Self::Output {
        self.ok_or("/:").bad_request_err(message)
    }

    fn not_found_err(self, message: &str) -> Self::Output {
        self.ok_or("/:").not_found_err(message)
    }

    fn forbidden_err(self, message: &str) -> Self::Output {
        self.ok_or("/:").forbidden_err(message)
    }

    fn database_err(self) -> Self::Output {
        self.ok_or("/:").database_err()
    }

    fn already_username_err(self, username: &str) -> Self::Output {
        self.ok_or("/:").already_username_err(username)
    }

    fn key_creation_err(self) -> Self::Output {
        self.ok_or("/:").key_creation_err()
    }

    fn invalid_token_err(self) -> Self::Output {
        self.ok_or("/:").invalid_token_err()
    }

    fn user_not_found_err(self) -> Self::Output {
        self.ok_or("/:").user_not_found_err()
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InternalServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BAdRequest(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let error = ErrorSchema::new(self.status_code().as_u16(), self.to_string());
        HttpResponse::build(self.status_code()).json(error)
    }
}

impl Responder for Error {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        self.error_response()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
