mod create;

use actix_web::{body::BoxBody, Responder};
pub use create::*;

use entity::todo::Status as TodoStatus;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

/// A todo schema
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TodoScheam {
    /// The id of the todo
    #[schema(example = "a8bfed8d-4f8b-4150-8ace-3f8916609eba")]
    pub uuid: Uuid,
    /// The title of the todo
    #[schema(example = "Todo title")]
    pub title: String,
    /// The status of the todo
    #[schema(value_type = String, example = "completed")]
    pub status: TodoStatus,
}

impl TodoScheam {
    /// Create a new todo
    pub fn new(uuid: Uuid, title: String, status: TodoStatus) -> Self {
        Self {
            uuid,
            title,
            status,
        }
    }

    /// Create a todo from a active todo model
    pub fn from_active_model(todo: entity::todo::ActiveModel) -> Self {
        Self::new(
            todo.uuid.unwrap(),
            todo.title.unwrap(),
            todo.status.unwrap(),
        )
    }
}

impl Responder for TodoScheam {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
    }
}
