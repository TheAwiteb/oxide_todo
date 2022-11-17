use actix_web::{post, web, Responder};
use sea_orm::DatabaseConnection;

use crate::schemas::auth::RegisterSchema;

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
