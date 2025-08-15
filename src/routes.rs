use actix_web::web::{self, delete, get, post, put};
use crate::handlers;

pub fn init_routes(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/todos")
            .route("", get().to(handlers::get_todos))
            .route("", post().to(handlers::create_todo))
            .route("/{id}", put().to(handlers::update_todo))
            .route("/{id}", delete().to(handlers::delete_todo))
    );
}