use actix_web::{body::BoxBody, Responder};
use entity::todo::Status as TodoStatus;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::api::todo::queries::{TodoFilters, TodoOrder, TodoOrderBy};

use super::TodoSchema;

/// The meta data of the todo list
#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
pub struct TodoListMetaSchema {
    /// The total number of todos
    #[schema(example = "1")]
    pub total: u64,
    /// The number of todos in the current page
    #[schema(example = "1")]
    pub count: u64,
    /// The offset of the current page
    #[schema(example = "0")]
    pub offset: u64,
    /// The limit of the current page
    #[schema(example = "10")]
    pub limit: u64,
    /// The status of todos in the current page
    /// Note: Will return `null` if the status is not set
    #[schema(value_type = Option<String>, example = "completed")]
    pub status: Option<TodoStatus>,
    /// The like title of todos in the current page
    /// Note: Will return `null` if the title is not set
    #[schema(example = "homework")]
    pub title: Option<String>,
    /// The order_by of todos in the current page
    #[schema(value_type = String, example = "updated_at")]
    pub order_by: TodoOrderBy,
    /// The order of todos in the current page
    #[schema(value_type = String, example = "asc")]
    pub order: TodoOrder,
}

/// List of todos, with filters
#[derive(ToSchema, Serialize, Deserialize, Debug, Clone)]
pub struct TodoListSchema {
    /// The list of todos
    #[schema(
        example = "[{\"uuid\": \"a8bfed8d-4f8b-4150-8ace-3f8916609eba\", \"title\": \"Todo title\", \"status\": \"completed\", \"created_at\": 1620000000, \"updated_at\": 1620000000}]"
    )]
    pub data: Vec<TodoSchema>,
    /// The meta data of the list
    #[serde(flatten)]
    pub meta: TodoListMetaSchema,
}

impl TodoListMetaSchema {
    /// Create a new todo list meta
    pub fn new(params: &TodoFilters, total: u64, count: u64) -> Self {
        Self {
            total,
            count,
            offset: params.offset(),
            limit: params.limit(),
            status: params.status.clone(),
            title: params.title.clone(),
            order_by: params.order_by(),
            order: params.order(),
        }
    }
}

impl TodoListSchema {
    /// Create a new todo list
    pub fn new(todos: Vec<TodoSchema>, params: &TodoFilters, total: u64) -> Self {
        let count = todos.len() as u64;
        Self {
            data: todos,
            meta: TodoListMetaSchema::new(params, total, count),
        }
    }
}

impl Responder for TodoListSchema {
    type Body = BoxBody;

    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse<Self::Body> {
        actix_web::HttpResponse::Ok().json(self)
    }
}
