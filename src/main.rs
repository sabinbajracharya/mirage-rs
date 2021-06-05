use actix_web::{App, HttpServer};

#[macro_use]
extern crate diesel;

extern crate dotenv;

#[macro_use]
extern crate diesel_migrations;

use dotenv::dotenv;

mod route;
mod models;
mod schema;
mod db;
mod error_handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    db::init();

    HttpServer::new(|| {
        App::new()
            .configure(route::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}