use crate::errors::{Error as ApiError, ErrorTrait, Result as ApiResult};

use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Select};
use uuid::Uuid;

/// Returns a unique UUID
/// ## Example
/// ```rust|no_run
/// // ...
/// let uuid = unique_uuid(TodoEntity::find(), TodoColumn::Uuid, db).await?;
/// // ...
/// ```
pub async fn unique_uuid<E>(
    select: Select<E>,
    column: impl ColumnTrait,
    db: &DatabaseConnection,
) -> ApiResult<Uuid>
where
    E: EntityTrait,
{
    // Counter of attempts to generate a unique uuid, to prevent an infinite loop
    let mut counter = 0;
    loop {
        counter += 1;
        let uuid = Uuid::new_v4();
        if select
            .clone()
            .filter(column.eq(uuid))
            .one(db)
            .await
            .database_err()?
            .is_none()
        {
            return Ok(uuid);
        } else if counter > 10 {
            return Err(ApiError::InternalServer(
                "Failed to generate a unique uuid".to_string(),
            ));
        }
    }
}
