use crate::auth::utils as auth_utils;
use crate::errors::{ErrorTrait, Result as ApiResult};
use crate::schemas::todo::TodoListSchema;
use crate::schemas::todo::TodoScheam;
use crate::todo::queries::TodoFilters;
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
            example = json!{{
                "data": [
                    {
                        "uuid": "b5a5d4e4-7d4e-4f4a-9f3d-3f3f3f3f3f3f",
                        "title": "Buy milk, eggs, and bread",
                        "status": "completed",
                        "created_at": 1615657387,
                        "updated_at": 1615657387,
                    },
                    {
                        "uuid": "b5a5d4e4-7d4e-4f4a-9f3d-4f4f4f4f4f4f",
                        "title": "Do the chemistry homework",
                        "status": "pending",
                        "created_at": 1615657397,
                        "updated_at": 1615657397,
                    }
                ],
                "meta": {
                    "total": 2,
                    "count": 2,
                    "limit": 10,
                    "offset": 0,
                    "status": null,
                    "title": null,
                    "order": "desc",
                    "order_by": "created_at"
                }
            }}
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
        .map(|todos| todos.into_iter().map(TodoScheam::from).collect())
        .map(|todos| TodoListSchema::new(todos, &params, total))
        .database_err()
}
