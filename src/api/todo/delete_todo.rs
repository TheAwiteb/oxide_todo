use actix_web::{
    delete,
    web::{self, Path},
    HttpRequest,
};
use sea_orm::{entity::ModelTrait, DatabaseConnection};
use uuid::Uuid;

use crate::{
    api::auth::utils::req_auth,
    api::todo::utils,
    errors::{ErrorTrait, Result as ApiResult},
    schemas::{message::MessageSchema, todo::TodoSchema, traits::OpenApiExample},
};

/// Delete a single todo by uuid.
#[utoipa::path(
    context_path = "/api/todos",
    params(
        (
            "uuid", description = "The uuid of the todo",
            example = "b5a5d4e4-7d4e-4f4a-9f3d-3f3f3f3f3f3f"
        )
    ),
    responses(
        (
            status = 200, description = "Delete a single todo by uuid", body = TodoScheam,
            example = json!(TodoSchema::example())
        ),
        (
            status = 404, description = "There is no todo with the given uuid", body = MessageSchema,
            example = json!(MessageSchema::new(404, "There is no todo with the given uuid"))
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
) -> ApiResult<TodoSchema> {
    let db = db.get_ref();
    let uuid = uuid.into_inner();
    let user = req_auth(req, db).await?;
    let todo = utils::find_todo_by_uuid(uuid, user.id, db).await?;
    todo.clone().delete(db).await.database_err()?;
    Ok(todo.into())
}
