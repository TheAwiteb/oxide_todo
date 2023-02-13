use actix_web::ResponseError;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::errors::Error as ApiError;

/// The schema for response error
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct ErrorSchema {
    /// The status code of the error
    pub status: u16,
    /// The error message
    pub message: String,
}

impl ErrorSchema {
    pub fn new(status: u16, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }
}

impl From<ApiError> for ErrorSchema {
    fn from(error: ApiError) -> Self {
        Self::new(error.status_code().as_u16(), error.to_string())
    }
}
