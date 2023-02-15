use actix_web::{patch, web, HttpRequest, Responder};
use chrono::Utc;
use entity::user::Model as UserModel;
use sea_orm::{ActiveModelTrait, DatabaseConnection, IntoActiveModel, Set};

use crate::api::auth::utils as auth_utils;
use crate::errors::{ErrorTrait, Result as ApiResult};
use crate::schemas::user::UserSchema;

/// Revoke a token by user, will return the new token
pub async fn revoke_token(db: &DatabaseConnection, user: UserModel) -> ApiResult<UserSchema> {
    let mut user = user.into_active_model();
    user.token_created_at = Set(Utc::now().naive_utc().timestamp());
    user.save(db)
        .await
        .database_err()
        .map(UserSchema::try_from_active_model)?
}

/// Revoke the previous tokens, will return the new token
#[utoipa::path(
    context_path = "/api/auth",
    responses(
        (
            status = 200, description = "Revoke the previous tokens and return a new token", body = UserSchema
        )
    ),
    tag = "Auth",
    security(("Bearer Token" = []))
)]
#[patch("/revoke")]
pub async fn revoke(req: HttpRequest, db: web::Data<DatabaseConnection>) -> impl Responder {
    let user = auth_utils::req_auth(req, &db).await?;
    revoke_token(&db, user).await
}
