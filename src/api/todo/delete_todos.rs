use actix_web::{delete, web, HttpRequest};
use entity::todo::{Column as TodoColumn, Entity as TodoEntity};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::{
    api::auth::utils::req_auth,
    errors::{ErrorTrait, Result as ApiResult},
    schemas::message::MessageSchema,
};

/// Delete all todos.
#[utoipa::path(
    context_path = "/api/todos",
    responses(
        (
            status = 200, description = "Delete a single todo by uuid", body = MessageSchema,
            example = json!{MessageSchema::new(200, "All todos deleted successfully")}
        )
    ),
    tag = "Todo",
    security(("Bearer Token" = []))
)]
#[delete("")]
pub async fn delete_todos(
    req: HttpRequest,
    db: web::Data<DatabaseConnection>,
) -> ApiResult<MessageSchema> {
    let db = db.get_ref();
    let user = req_auth(req, db).await?;

    TodoEntity::delete_many()
        .filter(TodoColumn::UserId.eq(user.id))
        .exec(db)
        .await
        .database_err()?;
    Ok(MessageSchema::new(200, "All todos deleted successfully"))
}
