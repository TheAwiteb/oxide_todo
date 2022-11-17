use serde::Serialize;

/// The schema for response error
#[derive(Debug, Serialize)]
pub struct ErrorSchema {
    pub status: u16,
    pub message: String,
}

impl ErrorSchema {
    pub fn new(status: u16, message: String) -> Self {
        Self { status, message }
    }
}
