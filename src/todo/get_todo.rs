use actix_web::{
    get,
    web::{self, Path},
    HttpRequest,
};
use entity::todo::{Column as TodoColumn, Entity as TodoEntity};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::{
    auth::utils::req_auth,
    errors::{ErrorTrait, Result as ApiResult},
    schemas::{message::MessageSchema, todo::TodoScheam},
};

/// Get a single todo by uuid.
#[utoipa::path(
    context_path = "/api/todos",
    params(
        (
            "uuid", description = "The uuid of the todo", Path,
            example = "b5a5d4e4-7d4e-4f4a-9f3d-3f3f3f3f3f3f"
        )
    ),
    responses(
        (
            status = 200, description = "Get a single todo by uuid", body = TodoScheam,
            example = json!{{
                "uuid": "b5a5d4e4-7d4e-4f4a-9f3d-3f3f3f3f3f3f",
                "title": "Buy milk, eggs, and bread",
                "status": "completed",
                "created_at": 1615657387,
                "updated_at": 1615657387,
            }}
        ),
        (
            status = 404, description = "There is no todo with the given uuid", body = MessageSchema,
            example = json!(MessageSchema::new(404, "There is no todo with the given uuid"))
        )
    ),
    tag = "Todo",
    security(("Bearer Token" = []))
)]
#[get("/{uuid}")]
pub async fn get_todo(
    req: HttpRequest,
    db: web::Data<DatabaseConnection>,
    uuid: Path<Uuid>,
) -> ApiResult<TodoScheam> {
    let db = db.get_ref();
    let uuid = uuid.into_inner();
    let user = req_auth(req, db).await?;

    TodoEntity::find()
        .filter(TodoColumn::Uuid.eq(uuid))
        .filter(TodoColumn::UserId.eq(user.id))
        .one(db)
        .await
        .database_err()?
        .map(TodoScheam::from)
        .not_found_err("There is no todo with the given uuid")
}
