use crate::errors::{Error as TodoError, Result as TodoResult, TodoError as TodoErrorTrait};

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
) -> TodoResult<Uuid>
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
            return Err(TodoError::InternalServer(
                "Failed to generate a unique uuid".to_string(),
            ));
        }
    }
}
