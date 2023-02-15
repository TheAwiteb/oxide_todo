use actix_web::{body::BoxBody, http::StatusCode, Responder, ResponseError};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::errors::Error as ApiError;

/// The schema for response error
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct MessageSchema {
    /// The status code of the error
    pub status: u16,
    /// The error message
    pub message: String,
}

impl MessageSchema {
    pub fn new(status: u16, message: impl Into<String>) -> Self {
        Self {
            status,
            message: message.into(),
        }
    }
}

impl Responder for MessageSchema {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::build(
            StatusCode::from_u16(self.status).expect("Invalid status code, should be 100-599"),
        )
        .content_type("application/json")
        .json(self)
    }
}

impl From<ApiError> for MessageSchema {
    fn from(error: ApiError) -> Self {
        Self::new(error.status_code().as_u16(), error.to_string())
    }
}
