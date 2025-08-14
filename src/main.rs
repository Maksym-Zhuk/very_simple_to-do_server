use std::env;

use actix_web::{web, App, HttpResponse, HttpServer};
use dotenvy::dotenv;
use log::info;

mod routes;
mod handlers;

#[actix_web::main]
async  fn main() {
    dotenv().ok();
    env_logger::init();

    let port: u16 = env::var("PORT").unwrap().parse().unwrap();

    info!("Server started in the port: {}", port);

    HttpServer::new(move || {
        App::new()
            .configure(routes::init_routes)
    })
    .bind(("0.0.0.0", port))
    .unwrap()
    .run()
    .await
    .unwrap()
}
