use actix_web::{web, HttpRequest, Responder};
use chrono::Utc;
use entity::user::Model as UserModel;
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel, Set};

use crate::auth::utils as auth_utils;
use crate::errors::{Result as TodoResult, TodoError as TodoErrorTrait};
use crate::schemas::user::UserSchema;

/// Revoke a token by user, will return the new token
pub async fn revoke_token(db: &DatabaseConnection, user: UserModel) -> TodoResult<UserSchema> {
    let mut user = user.into_active_model();
    user.last_revoke_token_at = Set(Some(Utc::now().naive_utc()));
    user.save(db)
        .await
        .database_err()
        .map(UserSchema::try_from_active_model)?
}

pub async fn revoke(req: HttpRequest, db: web::Data<DatabaseConnection>) -> impl Responder {
    let user = auth_utils::req_auth(req, &db).await?;
    revoke_token(&db, user).await
}
