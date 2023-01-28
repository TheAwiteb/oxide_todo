use actix_web::web;

pub mod create;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/todo").service(create::create));
}
