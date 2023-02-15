use crate::api::auth::utils::req_auth;
use crate::api::todo::utils;
use crate::errors::Result as ApiResult;
use crate::schemas::message::MessageSchema;
use crate::schemas::todo::{TodoContentSchema, TodoSchema};
use actix_web::{put, web, HttpRequest};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

/// Update a single todo by uuid, only the title and status can be updated.
#[utoipa::path(
    context_path = "/api/todos",
    request_body = TodoContentSchema,
    params(
        (
            "uuid", description = "The uuid of the todo", Path,
            example = "b5a5d4e4-7d4e-4f4a-9f3d-3f3f3f3f3f3f"
        )
    ),
    responses(
        (
            status = 200, description = "Update a single todo by uuid", body = TodoScheam,
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
#[put("/{uuid}")]
pub async fn update_todo(
    req: HttpRequest,
    payload: web::Json<TodoContentSchema>,
    uuid: web::Path<Uuid>,
    db: web::Data<DatabaseConnection>,
) -> ApiResult<TodoSchema> {
    let payload = payload.into_inner();
    let db = db.as_ref();
    let user = req_auth(req, db).await?;
    let todo = utils::find_todo_by_uuid(*uuid, user.id, db).await?;
    // If the title is not changed, then set it to None. Otherwise, set it to Some(payload.title)
    let todo_title = todo.title.ne(&payload.title).then_some(payload.title);
    utils::update_todo(todo, todo_title, Some(payload.status), db)
        .await
        .map(TodoSchema::from)
}
