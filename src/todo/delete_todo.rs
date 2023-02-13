use actix_web::{
    delete,
    web::{self, Path},
    HttpRequest,
};
use entity::todo::{Column as TodoColumn, Entity as TodoEntity};
use sea_orm::{entity::ModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{
    auth::utils::req_auth,
    errors::{ErrorTrait, Result as ApiResult},
    schemas::{errors::ErrorSchema, todo::TodoScheam},
};

/// Delete a single todo by uuid.
#[utoipa::path(
    context_path = "/api/todo",
    params(
        (
            "uuid", description = "The uuid of the todo", Path,
            example = "b5a5d4e4-7d4e-4f4a-9f3d-3f3f3f3f3f3f"
        )
    ),
    responses(
        (
            status = 200, description = "Delete a single todo by uuid", body = TodoScheam,
            example = json!{{
                "uuid": "b5a5d4e4-7d4e-4f4a-9f3d-3f3f3f3f3f3f",
                "title": "Buy milk, eggs, and bread",
                "status": "completed",
                "created_at": 1615657387,
                "updated_at": 1615657387,
            }}
        ),
        (
            status = 404, description = "There is no todo with the given uuid", body = ErrorSchema,
            example = json!(ErrorSchema::new(404, "There is no todo with the given uuid"))
        )
    ),
    tag = "Todo",
    security(("Bearer Token" = []))
)]
#[delete("/{uuid}")]
pub async fn delete_todo(
    req: HttpRequest,
    db: web::Data<DatabaseConnection>,
    uuid: Path<Uuid>,
) -> ApiResult<TodoScheam> {
    let db = db.get_ref();
    let uuid = uuid.into_inner();
    let user = req_auth(req, db).await?;

    let todo = TodoEntity::find()
        .filter(TodoColumn::Uuid.eq(uuid))
        .filter(TodoColumn::UserId.eq(user.id))
        .one(db)
        .await
        .database_err()?
        .not_found_err("There is no todo with the given uuid")?;

    todo.clone().delete(db).await.database_err()?;

    Ok(todo.into())
}
