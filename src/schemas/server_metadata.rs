use std::env;

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::api::todo;

/// Server metadata, helps clients to know what to expect from the server.
#[derive(Debug, ToSchema, Serialize, Deserialize, Clone)]
pub struct ServerMetadataSchema {
    /// The version of the server
    pub version: String,
    /// The name of the server
    pub name: String,
    /// The reatelimit burst size
    pub reatelimit_burst_size: u16,
    /// The reatelimit reset duration in seconds
    pub ratelimit_duration: u16,
    /// The API contact name
    pub contact_name: String,
    /// The API contact email
    pub contact_email: String,
    /// The API contact website
    pub contact_website: String,
    /// The maximum number of todos a user can have
    pub max_todos: u64,
    /// The maximum todo title length
    pub max_title_length: u64,
}

impl Default for ServerMetadataSchema {
    fn default() -> Self {
        const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");

        Self {
            version: SERVER_VERSION.to_owned(),
            name: env::var("API_NAME").unwrap_or_else(|_| "RESTful Todo API".to_owned()),
            reatelimit_burst_size: env::var("RATELIMIT_BURST_SIZE")
                .unwrap_or_else(|_| "30".to_owned())
                .parse()
                .expect("`RATELIMIT_BURST_SIZE` must be a number"),
            ratelimit_duration: env::var("RATE_LIMIT_PER_SECOND")
                .unwrap_or_else(|_| "60".to_owned())
                .parse()
                .expect("`RATELIMIT_DURATION` must be a number"),
            contact_name: env::var("API_CONTACT_NAME").expect("`API_CONTACT_NAME` must be set"),
            contact_email: env::var("API_CONTACT_EMAIL").expect("`API_CONTACT_EMAIL` must be set"),
            contact_website: env::var("API_CONTACT_URL").expect("`API_CONTACT_URL` must be set"),
            max_todos: todo::utils::max_todos_count(),
            max_title_length: todo::utils::max_todo_title_length(),
        }
    }
}
