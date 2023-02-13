use actix_web::web;

pub mod create;
pub mod list;
pub mod queries;
pub mod utils;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todo")
            .service(create::create)
            .service(list::list),
    );
}
