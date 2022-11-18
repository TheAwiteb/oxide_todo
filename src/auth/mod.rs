use actix_web::web;

pub mod login;
pub mod register;
pub mod revoke;
pub mod traits;
pub mod utils;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(register::register)
            .service(login::login)
            .service(revoke::revoke),
    );
}
