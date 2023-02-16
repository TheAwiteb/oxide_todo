use actix_web::{
    get,
    web::{self, Path},
    HttpRequest,
};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::{
    api::auth::utils::req_auth,
    api::todo::utils,
    errors::Result as ApiResult,
    schemas::{message::MessageSchema, todo::TodoSchema, traits::OpenApiExample},
};

/// Get a single todo by uuid.
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
            status = 200, description = "Get a single todo by uuid", body = TodoScheam,
            example = json!(TodoSchema::openapi_example())
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
) -> ApiResult<TodoSchema> {
    let db = db.get_ref();
    let uuid = uuid.into_inner();
    let user = req_auth(req, db).await?;

    utils::find_todo_by_uuid(uuid, user.id, db)
        .await
        .map(From::from)
}
