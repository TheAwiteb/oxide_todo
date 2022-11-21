use actix_web::{
    body::BoxBody, http::StatusCode, HttpRequest, HttpResponse, Responder, ResponseError,
};

use crate::schemas::errors::ErrorSchema;

#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("{0}")]
    InternalServer(String),
    #[error("{0}")]
    BAdRequest(String),
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Forbidden(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("Too many requests, retry in {0}s")]
    TooManyRequests(u64),
}

pub trait TodoError {
    type Output;

    fn server_err(self, message: &str) -> Self::Output;
    fn bad_request_err(self, message: &str) -> Self::Output;
    fn not_found_err(self, message: &str) -> Self::Output;
    fn unauthorized_err(self, message: &str) -> Self::Output;

    fn database_err(self) -> Self::Output;
    fn already_username_err(self, username: &str) -> Self::Output;
    fn key_creation_err(self) -> Self::Output;
    fn invalid_token_err(self) -> Self::Output;
    fn incorrect_user_err(self) -> Self::Output;
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

    fn unauthorized_err(self, message: &str) -> Self::Output {
        self.map_err(|_| Error::Unauthorized(message.to_string()))
    }

    fn database_err(self) -> Self::Output {
        self.server_err("Database error ):")
    }

    fn already_username_err(self, username: &str) -> Self::Output {
        self.bad_request_err(&format!("Username `{}` already exists", username))
    }

    fn key_creation_err(self) -> Self::Output {
        self.server_err("Error while creating the key ):")
    }

    fn invalid_token_err(self) -> Self::Output {
        self.unauthorized_err("The token is invalid")
    }

    fn incorrect_user_err(self) -> Self::Output {
        self.bad_request_err("The username or password is incorrect")
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

    fn unauthorized_err(self, message: &str) -> Self::Output {
        self.ok_or("/:").unauthorized_err(message)
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

    fn incorrect_user_err(self) -> Self::Output {
        self.ok_or("/:").incorrect_user_err()
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::InternalServer(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::BAdRequest(_) => StatusCode::BAD_REQUEST,
            Self::NotFound(_) => StatusCode::NOT_FOUND,
            Self::Forbidden(_) => StatusCode::FORBIDDEN,
            Self::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Self::TooManyRequests(_) => StatusCode::TOO_MANY_REQUESTS,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorSchema::from(self.clone()))
    }
}

impl Responder for Error {
    type Body = BoxBody;

    fn respond_to(self, _: &HttpRequest) -> HttpResponse<Self::Body> {
        self.error_response()
    }
}

pub type Result<T> = std::result::Result<T, Error>;
