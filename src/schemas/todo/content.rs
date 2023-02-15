use entity::todo::Status as TodoStatus;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// The content of the todo, used for creating a todo or updating a todo
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct TodoContentSchema {
    /// The title of the todo. It must be unique
    #[schema(example = "Todo title")]
    pub title: String,
    /// The status of the todo. It must be one of the following: `completed`, `progress`, `pending` or `canceled`
    #[schema(value_type = String, example = "pending")]
    pub status: TodoStatus,
}
