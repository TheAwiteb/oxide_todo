use actix_web::{post, web, Responder};
use sea_orm::DatabaseConnection;

use crate::schemas::{auth::LoginSchema, errors::ErrorSchema};

/// Login a user
///
/// This endpoint will return a new token for the user. if the user does not exist, it will return an error.
/// To revoke a previous tokens, use the revoke endpoint `/api/auth/revoke`.
#[utoipa::path(
    context_path = "/api/auth",
    request_body = LoginSchema,
    responses(
        (
            status = 200, description = "Login successfully and return a new token", body = UserSchema
        ),
        (
            status = 400, description = "The username or password is incorrect", body = ErrorSchema,
            example = json!(ErrorSchema::new(400, "The username or password is incorrect"))
        ),
    ),
    tag = "Auth"
)]
#[post("/login")]
pub async fn login(
    db: web::Data<DatabaseConnection>,
    payload: web::Json<LoginSchema>,
) -> impl Responder {
    let db = db.get_ref();
    log::info!("Logging in user: {}", payload.username);
    payload.login(db).await
}
