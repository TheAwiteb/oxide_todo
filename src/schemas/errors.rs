use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
