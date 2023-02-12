use std::str::FromStr;

use chrono::Utc;
use entity::{
    todo::{
        ActiveModel as NewTodo, Column as TodoColumn, Entity as TodoEntity, Status as TodoStatus,
    },
    user::Model as UserModel,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::schemas::todo::TodoScheam as TodoSchema;
use crate::{
    errors::{Error as TodoError, Result as TodoResult, TodoError as TodoErrorTrait},
    todo::utils::unique_uuid,
};

/// The schema for creating a todo
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct CreateTodoSchema {
    /// The title of the todo
    #[schema(example = "Todo title")]
    pub title: String,
    /// The status of the todo
    #[schema(example = "pending")]
    pub status: String,
}

impl CreateTodoSchema {
    /// Createing a new todo
    pub async fn create(
        &self,
        db: &DatabaseConnection,
        author: &UserModel,
    ) -> TodoResult<TodoSchema> {
        if self.title.is_empty() {
            return Err(TodoError::BAdRequest("The todo title is empty".to_string()));
        } else if TodoEntity::find()
            .filter(
                TodoColumn::Title
                    .eq(self.title.clone())
                    .and(TodoColumn::UserId.eq(author.id)),
            )
            .one(db)
            .await
            .database_err()?
            .is_some()
        {
            return Err(TodoError::BAdRequest(format!(
                "The todo `{}` is already exists",
                self.title
            )));
        }

        let current_time = Utc::now().naive_utc().timestamp();
        let uuid = unique_uuid(TodoEntity::find(), TodoColumn::Uuid, db).await?;

        NewTodo {
            uuid: Set(uuid),
            title: Set(self.title.clone()),
            status: Set(TodoStatus::from_str(&self.status).map_err(TodoError::BAdRequest)?),
            created_at: Set(current_time),
            updated_at: Set(current_time),
            user_id: Set(author.id),
            ..Default::default()
        }
        .save(db)
        .await
        .map(From::from)
        .database_err()
    }
}
