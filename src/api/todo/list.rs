use crate::api::auth::utils as auth_utils;
use crate::api::todo::queries::TodoFilters;
use crate::errors::{ErrorTrait, Result as ApiResult};
use crate::schemas::{todo::TodoListSchema, traits::OpenApiExample};
use actix_web::{get, web, HttpRequest};
use entity::todo::{Column as TodoColumn, Entity as TodoEntity};
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect,
};

/// list todos, filterable by status, title, limit, offset, order, and order_by.
#[utoipa::path(
    context_path = "/api/todos",
    params(TodoFilters),
    responses(
        (
            status = 200, description = "List todos", body = TodoListSchema,
            example = json!(TodoListSchema::example())
        )
    ),
    tag = "Todo",
    security(("Bearer Token" = []))
)]
#[get("")]
pub async fn list(
    req: HttpRequest,
    db: web::Data<DatabaseConnection>,
    params: web::Query<TodoFilters>,
) -> ApiResult<TodoListSchema> {
    let db = db.get_ref();
    let user = auth_utils::req_auth(req, db).await?;
    let mut query = TodoEntity::find().filter(TodoColumn::UserId.eq(user.id));

    if let Some(title) = &params.title {
        query = query.filter(TodoColumn::Title.like(title));
    }
    if let Some(status) = &params.status {
        query = query.filter(TodoColumn::Status.eq(status.clone()));
    }

    let total = query.clone().count(db).await.database_err()?;
    query
        .order_by(TodoColumn::from(params.order_by()), params.order().into())
        .limit(params.limit())
        .offset(params.offset())
        .all(db)
        .await
        .map(|todos| {
            TodoListSchema::new(todos.into_iter().map(From::from).collect(), &params, total)
        })
        .database_err()
}
