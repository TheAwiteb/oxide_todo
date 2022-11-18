use actix_web::{post, web, Responder};
use sea_orm::DatabaseConnection;

use crate::schemas::{auth::RegisterSchema, errors::ErrorSchema};

/// Register a new user, will return the new token for the user.
///
/// ### Note:
/// - The username should be unique, if not, will return `400 Bad Request`
#[utoipa::path(
    context_path = "/api/auth",
    request_body = RegisterSchema,
    responses(
        (
            status = 201, description = "Register successfully and return a new token", body = UserSchema
        ),
        (
            status = 400, description = "The username is not unique", body = ErrorSchema,
            example = json!(ErrorSchema::new(400, "Username `Awiteb` already exists"))
        ),
    ),
    tag = "Auth"
)]
#[post("/register")]
pub async fn register(
    db: web::Data<DatabaseConnection>,
    payload: web::Json<RegisterSchema>,
) -> impl Responder {
    log::info!("Registering user: {}", payload.username);
    // 201 Created response code
    payload
        .create(db.get_ref())
        .await
        .map(|user| user.with_code(201))
}
