use actix_web::web;

pub mod create;
pub mod delete_todo;
pub mod delete_todos;
pub mod get_todo;
pub mod list;
pub mod queries;
pub mod update;
pub mod utils;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todos")
            .service(create::create)
            .service(get_todo::get_todo)
            .service(update::update_todo)
            .service(delete_todo::delete_todo)
            .service(list::list)
            .service(delete_todos::delete_todos),
    );
}
