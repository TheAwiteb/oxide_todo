pub mod traits;
pub mod utils;
use actix_web::web;

mod login;
mod register;
mod revoke;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(register::register)
            .service(login::login)
            .service(revoke::revoke),
    );
}
