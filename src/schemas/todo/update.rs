use entity::todo::Status as TodoStatus;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// The schema used to update a todo, supports null values for unchanged fields
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct UpdateTodoSchema {
    /// The title of the todo, can be `null` to keep the original title
    #[schema(example = "Todo title")]
    pub title: Option<String>,
    /// The status of the todo, can be `null` to keep the original status
    #[schema(value_type = Option<String>, example = "completed")]
    pub status: Option<TodoStatus>,
}

impl Default for UpdateTodoSchema {
    fn default() -> Self {
        Self {
            title: Some("Todo title".to_string()),
            status: None,
        }
    }
}
