use actix_web::{get, post, web, HttpResponse, Responder};
use serde_json::json;

use crate::error_handler::CustomError;
use crate::models::{Endpoint, NewEndpoint, Content, NewContent, Allow, NewAllow};

use actix_web_static_files::ResourceFiles;
include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[post("/endpoint")]
async fn insert_endpoint(endpoint: web::Json<NewEndpoint>) -> Result<HttpResponse, CustomError> {
    let new_endpoint = Endpoint::create(endpoint.into_inner()).await?;
    Ok(HttpResponse::Ok().json(new_endpoint))
}

#[get("/endpoints")]
async fn get_all_endpoints() -> Result<HttpResponse, CustomError>  {
    let endpoints = Endpoint::find_all().await?;
    Ok(HttpResponse::Ok().json(endpoints))
}

#[get("/endpoint/{id}/contents")]
async fn get_all_content_from_endpoint_id(web::Path((id)): web::Path<(i32)>) -> Result<HttpResponse, CustomError> {
    let response = Content::find_all_from_endpoint_id(id).await?;
    Ok(HttpResponse::Ok().json(response))
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

#[post("/allow")]
async fn insert_allow(allow: web::Json<NewAllow>) -> Result<HttpResponse, CustomError> {
    let response = Allow::create(allow.into_inner()).await?;
    Ok(HttpResponse::Ok().json(response))
}

#[get("/allows")]
async fn get_all_allow() -> Result<HttpResponse, CustomError> {
    let allows = Allow::find_all().await?;
    Ok(HttpResponse::Ok().json(allows))
}

#[get("/dashboard")]
async fn show_dashboard() -> Result<HttpResponse, CustomError> {
    Ok(HttpResponse::Ok()
        .body(include_str!("../static/index.html")))
}

pub fn init_routes(config: &mut web::ServiceConfig) {
    config.service(insert_endpoint);
    config.service(get_all_endpoints);

    config.service(insert_content);
    config.service(get_all_content);
    config.service(get_all_content_from_endpoint_id);

    config.service(insert_allow);
    config.service(get_all_allow);

    config.service(show_dashboard);
    let generated = generate();
    config.service(ResourceFiles::new("/_app", generated));
}