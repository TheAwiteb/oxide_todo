use actix_web::{patch, web, HttpRequest, Responder};
use chrono::Utc;
use entity::user::Model as UserModel;
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel, Set};

use crate::auth::utils as auth_utils;
use crate::errors::{Result as TodoResult, TodoError as TodoErrorTrait};
use crate::schemas::errors::ErrorSchema;
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

/// Revoke the previous tokens, will return the new token
///
/// ### Note:
/// - Set the `Authorization` header with the old token, if not, will return `400 Bad Request`
/// - Token should start with `Bearer `, if not, will return `400 Bad Request`
/// - The token should be valid, if not, will return `401 Unauthorized`
/// - The token should not be revoked, if not, will return `403 Forbidden`
#[utoipa::path(
    context_path = "/api/auth",
    responses(
        (
            status = 200, description = "Revoke the previous tokens and return a new token", body = UserSchema
        ),
        (
            status = 400, description = "Unset `Authorization` header or token dose't start with `Bearer `", body = ErrorSchema, 
            example = json!(ErrorSchema::new(400, "The token is invalid"))),
        (
            status = 401, description = "The token is invalid", body = ErrorSchema,
            example = json!(ErrorSchema::new(401, "The token is invalid"))
        ),
        (
            status = 403, description = "The token is revoked", body = ErrorSchema,
            example = json!(ErrorSchema::new(403, "Token has been revoked"))
        ),
    ),
    tag = "Auth",
    security(("Bearer Token" = []))
)]
#[patch("/revoke")]
pub async fn revoke(req: HttpRequest, db: web::Data<DatabaseConnection>) -> impl Responder {
    let user = auth_utils::req_auth(req, &db).await?;
    revoke_token(&db, user).await
}
