use actix_web::web;
use crate::handlers;

pub fn init_routes(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/todos")
    );
}