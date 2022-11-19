use actix_governor::Governor;
use actix_web::web;

pub mod login;
pub mod register;
pub mod revoke;
pub mod traits;
pub mod utils;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    let bearer_limit_config = crate::ratelimit::init_bearer();
    let ip_limit_config = crate::ratelimit::init_ip();

    cfg.service(
        web::scope("/auth")
            .service(
                // Endpoints that dosn't require authentication
                web::scope("")
                    .wrap(Governor::new(&ip_limit_config))
                    .service(register::register)
                    .service(login::login),
            )
            .service(
                // Endpoints that require authentication
                web::scope("")
                    .wrap(Governor::new(&bearer_limit_config))
                    .service(revoke::revoke),
            ),
    );
}
