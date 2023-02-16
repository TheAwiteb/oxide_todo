use entity::todo::Column as TodoColumn;
use entity::todo::Status as TodoStatus;
use sea_orm::query::Order;
use serde::{Deserialize, Serialize};
use utoipa::IntoParams;

/// The order_by filter
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum TodoOrderBy {
    /// Order by created_at
    #[default]
    CreatedAt,
    /// Order by updated_at
    UpdatedAt,
}

/// The order filter
#[derive(Default, Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TodoOrder {
    /// Ascending order, Older first
    Older,
    /// Descending order, Newer first
    #[default]
    Newer,
}

/// Todo filters for querying
#[derive(IntoParams, Deserialize, Serialize, Debug, Clone)]
pub struct TodoFilters {
    /// Filter by status (`completed`, `pending`, `progress`, `cancelled`) (default: all)
    #[param(value_type = Option<String>, example = "completed")]
    pub status: Option<TodoStatus>,
    /// Filter by title (default: all)
    #[param(example = "homework")]
    pub title: Option<String>,
    /// Order the todos by (`created_at` or `updated_at`) (default: `created_at`
    #[param(value_type = Option<String>, example = "updated_at")]
    pub order_by: Option<TodoOrderBy>,
    /// Order the todos (`older` or `newer`) (default: `newer`)
    #[param(value_type = Option<String>, example = "newer")]
    pub order: Option<TodoOrder>,
    /// Offset the number of todos (default: `0`)
    #[param(example = "0")]
    pub offset: Option<u64>,
    /// Limit the number of todos (default: `10`)
    #[param(example = "10")]
    pub limit: Option<u64>,
}

impl TodoFilters {
    /// Returns the order_by filter
    /// Note: Will return `CreatedAt` if the filter is not set
    pub fn order_by(&self) -> TodoOrderBy {
        self.order_by.clone().unwrap_or_default()
    }

    /// Returns the order filter
    /// Note: Will return `Desc` if the filter is not set
    pub fn order(&self) -> TodoOrder {
        self.order.clone().unwrap_or_default()
    }

    /// Returns the offset filter
    /// Note: Will return `0` if the filter is not set
    pub fn offset(&self) -> u64 {
        self.offset.unwrap_or(0)
    }

    /// Returns the limit filter
    /// Note: Will return `10` if the filter is not set
    pub fn limit(&self) -> u64 {
        self.limit.unwrap_or(10)
    }
}

impl Default for TodoFilters {
    fn default() -> Self {
        Self {
            status: None,
            title: None,
            order_by: Some(TodoOrderBy::default()),
            order: Some(TodoOrder::default()),
            offset: Some(0),
            limit: Some(10),
        }
    }
}

impl From<TodoOrder> for Order {
    fn from(order: TodoOrder) -> Self {
        match order {
            TodoOrder::Older => Self::Asc,
            TodoOrder::Newer => Self::Desc,
        }
    }
}

impl From<TodoOrderBy> for TodoColumn {
    fn from(order_by: TodoOrderBy) -> Self {
        match order_by {
            TodoOrderBy::CreatedAt => Self::CreatedAt,
            TodoOrderBy::UpdatedAt => Self::UpdatedAt,
        }
    }
}
