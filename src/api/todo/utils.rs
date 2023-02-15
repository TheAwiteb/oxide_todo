use std::env;

use crate::{
    errors::{Error as ApiError, ErrorTrait, Result as ApiResult},
    schemas::todo::{TodoContentSchema, TodoScheam},
};
use chrono::Utc;
use entity::todo::{
    ActiveModel as NewTodo, Column as TodoColumn, Entity as TodoEntity, Model as TodoModel,
    Status as TodoStatus,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Select, Set,
};
use uuid::Uuid;

/// Returns a unique UUID
/// ## Example
/// ```rust|no_run
/// // ...
/// let uuid = unique_uuid(TodoEntity::find(), TodoColumn::Uuid, db).await?;
/// // ...
/// ```
pub async fn unique_uuid<E>(
    select: Select<E>,
    column: impl ColumnTrait,
    db: &DatabaseConnection,
) -> ApiResult<Uuid>
where
    E: EntityTrait,
{
    // Counter of attempts to generate a unique uuid, to prevent an infinite loop
    let mut counter = 0;
    loop {
        counter += 1;
        let uuid = Uuid::new_v4();
        if select
            .clone()
            .filter(column.eq(uuid))
            .one(db)
            .await
            .database_err()?
            .is_none()
        {
            return Ok(uuid);
        } else if counter > 10 {
            return Err(ApiError::InternalServer(
                "Failed to generate a unique uuid".to_string(),
            ));
        }
    }
}

/// Returns the maximum number of todos that can be created by a user
pub fn max_todos_count() -> u64 {
    env::var("MAXIMUM_TODO_PER_USER")
        .unwrap_or_else(|_| "500".to_string())
        .parse()
        .unwrap_or(500)
}

/// Returns the maximum todo title length that can be created by a user
pub fn max_todo_title_length() -> u64 {
    env::var("MAXIMUM_TODO_TITLE_LENGTH")
        .unwrap_or_else(|_| "100".to_string())
        .parse()
        .unwrap_or(100)
}

/// Returns whether if there is a todo with the given title and user id
pub async fn is_todo_title_exists(
    title: &str,
    user_id: u32,
    db: &DatabaseConnection,
) -> ApiResult<bool> {
    Ok(TodoEntity::find()
        .filter(
            TodoColumn::Title
                .eq(title)
                .and(TodoColumn::UserId.eq(user_id)),
        )
        .one(db)
        .await
        .database_err()?
        .is_some())
}

/// Returns a todo by uuid, if the todo is not found, returns an error 404
pub async fn find_todo_by_uuid(
    uuid: Uuid,
    user_id: u32,
    db: &DatabaseConnection,
) -> ApiResult<TodoModel> {
    TodoEntity::find()
        .filter(TodoColumn::UserId.eq(user_id))
        .filter(TodoColumn::Uuid.eq(uuid))
        .one(db)
        .await
        .database_err()?
        .not_found_err("There is no todo with the given uuid")
}

/// Update a todo, if the title is empty or the todo with the same title already exists, returns an error 400
pub async fn update_todo(
    todo: TodoModel,
    title: Option<String>,
    status: Option<TodoStatus>,
    db: &DatabaseConnection,
) -> ApiResult<TodoModel> {
    if let Some(title) = &title {
        if title.is_empty() {
            return Err(ApiError::BAdRequest("The todo title is empty".to_string()));
        } else if is_todo_title_exists(title, todo.user_id, db).await? {
            return Err(ApiError::BAdRequest(format!(
                "The todo `{}` is already exists",
                title
            )));
        } else if title.chars().count() > max_todo_title_length() as usize {
            return Err(ApiError::BAdRequest(format!(
                "The todo title length must be less than {}",
                max_todo_title_length()
            )));
        }
    }
    NewTodo {
        updated_at: Set(Utc::now().naive_utc().timestamp()),
        title: Set(title.unwrap_or_else(|| todo.title.clone())),
        status: Set(status.unwrap_or_else(|| todo.status.clone())),
        ..todo.into()
    }
    .update(db)
    .await
    .database_err()
}

/// Createing a new todo, if the title is empty or the todo with the same title already exists, returns an error 400
pub async fn create_todo(
    db: &DatabaseConnection,
    todo_content: TodoContentSchema,
    user_id: u32,
) -> ApiResult<TodoScheam> {
    if todo_content.title.is_empty() {
        return Err(ApiError::BAdRequest("The todo title is empty".to_string()));
    } else if is_todo_title_exists(&todo_content.title, user_id, db).await? {
        return Err(ApiError::BAdRequest(format!(
            "The todo `{}` is already exists",
            todo_content.title
        )));
    } else if todo_content.title.chars().count() > max_todo_title_length() as usize {
        return Err(ApiError::BAdRequest(format!(
            "The todo title length must be less than {}",
            max_todo_title_length()
        )));
    } else if TodoEntity::find()
        .filter(TodoColumn::UserId.eq(user_id))
        .count(db)
        .await
        .database_err()?
        > max_todos_count()
    {
        return Err(ApiError::BAdRequest(format!(
            "The maximum number of todos is {}",
            max_todos_count()
        )));
    }

    let current_time = Utc::now().naive_utc().timestamp();
    let uuid = unique_uuid(TodoEntity::find(), TodoColumn::Uuid, db).await?;

    NewTodo {
        uuid: Set(uuid),
        title: Set(todo_content.title.clone()),
        status: Set(todo_content.status),
        created_at: Set(current_time),
        updated_at: Set(current_time),
        user_id: Set(user_id),
        ..Default::default()
    }
    .save(db)
    .await
    .map(From::from)
    .database_err()
}
