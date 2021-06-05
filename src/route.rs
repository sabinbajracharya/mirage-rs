use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

use crate::error_handler::CustomError;
use crate::models::{Endpoint, NewEndpoint, Content, NewContent};

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

#[post("/content")]
async fn insert_content(content: web::Json<NewContent>) -> Result<HttpResponse, CustomError> {
    let response = Content::create(content.into_inner()).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[get("/contents")]
async fn get_all_content() -> Result<HttpResponse, CustomError> {
    let contents = Content::find_all().await?;
    Ok(HttpResponse::Ok().json(contents))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(insert_endpoint);
    config.service(get_all_endpoints);
    config.service(insert_content);
    config.service(get_all_content);
}