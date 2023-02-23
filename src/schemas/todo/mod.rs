mod content;
mod list;
mod update;

use actix_web::{body::BoxBody, Responder};
pub use {content::*, list::*, update::*};

use entity::todo::Status as TodoStatus;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// A todo schema
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TodoSchema {
    /// The id of the todo
    #[schema(value_type = String, example = "a8bfed8d-4f8b-4150-8ace-3f8916609eba")]
    pub uuid: Uuid,
    /// The title of the todo
    #[schema(example = "Todo title")]
    pub title: String,
    /// The status of the todo
    #[schema(value_type = String, example = "completed")]
    pub status: TodoStatus,
    /// The created time of the todo (Unix timestamp)
    #[schema(example = "1620000000")]
    pub created_at: i64,
    /// The updated time of the todo (Unix timestamp)
    /// If the todo is not updated, this value is equal to `created_at`
    #[schema(example = "1620000000")]
    pub updated_at: i64,
}

impl TodoSchema {
    /// Create a new todo
    pub fn new(
        uuid: Uuid,
        title: String,
        status: TodoStatus,
        created_at: i64,
        updated_at: i64,
    ) -> Self {
        Self {
            uuid,
            title,
            status,
            created_at,
            updated_at,
        }
    }
}

impl Default for TodoSchema {
    fn default() -> Self {
        Self::new(
            Uuid::new_v4(),
            "Todo title".to_string(),
            TodoStatus::Completed,
            1620000000,
            1620000000,
        )
    }
}

impl From<entity::todo::ActiveModel> for TodoSchema {
    fn from(todo: entity::todo::ActiveModel) -> Self {
        Self::new(
            todo.uuid.unwrap(),
            todo.title.unwrap(),
            todo.status.unwrap(),
            todo.created_at.unwrap(),
            todo.updated_at.unwrap(),
        )
    }
}

impl From<entity::todo::Model> for TodoSchema {
    fn from(todo: entity::todo::Model) -> Self {
        Self::new(
            todo.uuid,
            todo.title,
            todo.status,
            todo.created_at,
            todo.updated_at,
        )
    }
}

impl Responder for TodoSchema {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
    }
}
