use actix_web::web;

pub mod auth;
pub mod todo;

/// Initialize the api routes, all the routes are under `/api`
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .configure(auth::init_routes)
            .configure(todo::init_routes),
    );
}
