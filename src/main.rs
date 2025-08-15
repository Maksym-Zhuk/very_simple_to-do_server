use std::env;

use actix_web::{web::{self}, App, HttpServer};
use dotenvy::dotenv;
use log::info;
use sea_orm::Database;

mod routes;
mod handlers;
mod entities;

#[actix_web::main]
async  fn main() {
    dotenv().ok();
    env_logger::init();

    let port: u16 = env::var("PORT").unwrap().parse().unwrap();
    
    info!("Connected to database...");

    let db = Database::connect(env::var("DATABASE_URL").unwrap()).await.expect("Failed to connect to database");

    info!("Server started in the port: {}", port);

    HttpServer::new(move || {
        App::new()
        .app_data(web::Data::new(db.clone()))
        .configure(routes::init_routes)
        .default_service(web::to(handlers::not_found))
    })
    .bind(("0.0.0.0", port))
    .unwrap()
    .run()
    .await
    .unwrap()
}
