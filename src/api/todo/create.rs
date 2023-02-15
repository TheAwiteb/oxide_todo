use crate::api::auth::utils as auth_utils;
use crate::api::todo::utils;
use crate::errors::Result as ApiResult;
use crate::schemas::todo::TodoScheam;
use crate::schemas::{message::MessageSchema, todo::TodoContentSchema};
use actix_web::{post, web, HttpRequest};
use sea_orm::DatabaseConnection;

/// Create a new todo.
/// Note: Check the `TodoContentSchema` schema (It's the request body)
#[utoipa::path(
    context_path = "/api/todos",
    request_body = TodoContentSchema,
    responses(
        (
            status = 200, description = "Create a new todo", body = TodoSchema,
            example = json!{{
                "uuid": "b5a5d4e4-7d4e-4f4a-9f3d-3f3f3f3f3f3f",
                "title": "Buy milk, eggs, and bread",
                "status": "completed",
                "created_at": 1615657387,
                "updated_at": 1615657387,
            }}
        ),
        (
            status = 400, description = "The title is empty", body = MessageSchema,
            example = json!(MessageSchema::new(400, "The todo title is empty"))
        ),
        (
            status = 400, description = "The status is invalid", body = MessageSchema,
            example = json!(MessageSchema::new(400, "The status `{status}` is invalid"))
        ),
        (
            status = 400, description = "Dubplicate todo", body = MessageSchema,
            example = json!(MessageSchema::new(400, "The todo `{title}` already exists"))
        ),
    ),
    tag = "Todo",
    security(("Bearer Token" = []))
)]
#[post("")]
pub async fn create(
    req: HttpRequest,
    db: web::Data<DatabaseConnection>,
    payload: web::Json<TodoContentSchema>,
) -> ApiResult<TodoScheam> {
    log::info!("Creating a new todo: {}", payload.title);
    let db = db.get_ref();
    let user = auth_utils::req_auth(req, db).await?;
    let payload = payload.into_inner();

    utils::create_todo(db, payload, user.id)
        .await
        .map(From::from)
}
