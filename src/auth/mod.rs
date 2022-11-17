pub mod traits;
pub mod utils;
use actix_web::web;

mod login;
mod register;
mod revoke;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register::register))
            .route("/login", web::post().to(login::login))
            .route("/revoke", web::patch().to(revoke::revoke)),
    );
}
