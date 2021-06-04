use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

use crate::models::{Endpoint, NewEndpoint};

#[post("/endpoint")]
async fn insert_endpoint(endpoint: web::Json<NewEndpoint>) -> impl Responder {
    let new_endpoint = Endpoint::create(endpoint.into_inner()).await.unwrap();
    HttpResponse::Ok().json(new_endpoint)
}

#[get("/endpoints")]
async fn get_all_endpoints() -> impl Responder {
    let endpoints = Endpoint::find_all().await.unwrap();
    HttpResponse::Ok().json(endpoints)
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(insert_endpoint);
    config.service(get_all_endpoints);
}