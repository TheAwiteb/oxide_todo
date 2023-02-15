use actix_web::web;

pub mod auth;
pub mod server_metadata;
pub mod todo;

/// Initialize the api routes, all the routes are under `/api`
pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(server_metadata::get_server_metadata)
            .configure(auth::init_routes)
            .configure(todo::init_routes),
    );
}
