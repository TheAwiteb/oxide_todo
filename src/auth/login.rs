use actix_web::{web, Responder};
use sea_orm::DatabaseConnection;

use crate::schemas::auth::LoginSchema;

pub async fn login(
    db: web::Data<DatabaseConnection>,
    payload: web::Json<LoginSchema>,
) -> impl Responder {
    let db = db.get_ref();
    log::info!("Logging in user: {}", payload.username);
    payload.login(db).await
}
