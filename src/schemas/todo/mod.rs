mod create;

use actix_web::{body::BoxBody, Responder};
pub use create::*;

use entity::todo::Status as TodoStatus;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// A todo schema
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TodoScheam {
    /// The id of the todo
    #[schema(example = 1)]
    pub id: u32,
    /// The title of the todo
    #[schema(example = "Todo title")]
    pub title: String,
    /// The status of the todo
    #[schema(value_type = String, example = "completed")]
    pub status: TodoStatus,
}

impl TodoScheam {
    /// Create a new todo
    pub fn new(id: u32, title: String, status: TodoStatus) -> Self {
        Self { id, title, status }
    }

    /// Create a todo from a active todo model
    pub fn from_active_model(todo: entity::todo::ActiveModel) -> Self {
        Self::new(todo.id.unwrap(), todo.title.unwrap(), todo.status.unwrap())
    }
}

impl Responder for TodoScheam {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
    }
}
