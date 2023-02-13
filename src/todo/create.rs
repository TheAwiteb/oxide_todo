use crate::auth::utils as auth_utils;
use crate::schemas::{errors::ErrorSchema, todo::CreateTodoSchema};
use actix_web::{post, web, HttpRequest, Responder};
use sea_orm::DatabaseConnection;

/// Create a new todo.
/// Available status:
/// - `completed`: Completed todo
/// - `pending`: Pending todo
/// - `progress`: Progress todo
/// - `cancelled`: Cancelled todo
#[utoipa::path(
    context_path = "/api/todos",
    request_body = CreateTodoSchema,
    responses(
        (
            status = 201, description = "Create a new todo", body = TodoSchema,
            example = json!{{
                "uuid": "b5a5d4e4-7d4e-4f4a-9f3d-3f3f3f3f3f3f",
                "title": "Buy milk, eggs, and bread",
                "status": "completed",
                "created_at": 1615657387,
                "updated_at": 1615657387,
            }}
        ),
        (
            status = 400, description = "The title is empty", body = ErrorSchema,
            example = json!(ErrorSchema::new(400, "The todo title is empty"))
        ),
        (
            status = 400, description = "The status is invalid", body = ErrorSchema,
            example = json!(ErrorSchema::new(400, "The status `{status}` is invalid"))
        ),
        (
            status = 400, description = "Dubplicate todo", body = ErrorSchema,
            example = json!(ErrorSchema::new(400, "The todo `{title}` already exists"))
        ),
    ),
    tag = "Todo",
    security(("Bearer Token" = []))
)]
#[post("")]
pub async fn create(
    req: HttpRequest,
    db: web::Data<DatabaseConnection>,
    payload: web::Json<CreateTodoSchema>,
) -> impl Responder {
    log::info!("Creating a new todo: {}", payload.title);
    let db = db.get_ref();
    let user = auth_utils::req_auth(req, db).await?;
    payload.create(db, &user).await
}
