use actix_web::{http::StatusCode, post, web, Responder};
use sea_orm::DatabaseConnection;

use crate::schemas::{
    auth::RegisterSchema, message::MessageSchema, traits::OpenApiExample, user::UserSchema,
};

/// Register a new user, will return the new token for the user.
///
/// ### Note:
/// - The username should be unique, if not, will return `400 Bad Request`
#[utoipa::path(
    context_path = "/api/auth",
    request_body = RegisterSchema,
    responses(
        (
            status = 201, description = "Register successfully and return a new token", body = UserSchema,
            example = json!(UserSchema::openapi_example())
        ),
        (
            status = 400, description = "The username is not unique", body = MessageSchema,
            example = json!(MessageSchema::new(400, "Username `Awiteb` already exists"))
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
    payload
        .create(db.get_ref())
        .await
        .map(|user| user.with_code(StatusCode::CREATED))
}
