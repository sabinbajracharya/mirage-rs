
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world II!")
}

pub fn init_routes(config: &mut web::ServiceConfig) {
  config.service(hello);
}