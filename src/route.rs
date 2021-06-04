use actix_web::{get, post, web, HttpResponse, Responder};
use diesel::prelude::*;

use crate::models::{Api, NewApi};
use crate::schema;
use crate::db::connection;

#[get("/")]
async fn hello() -> impl Responder {
	use schema::apis::dsl::*;

	let connection = connection().unwrap();

	let results = apis
        .limit(5)
        .load::<Api>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} posts", results.len());
    for api in results {
        println!("{}", api.status_code);
        println!("----------\n");
        println!("{}", api.body);
    }

	HttpResponse::Ok().body("Hello world II!")
}

#[get("/insert")]
async fn insert() -> impl Responder {
	let conn = connection().unwrap();

	use schema::apis;

    let new_api = NewApi {
		pid: &123,
		body: "A body from 3rd dimension",
		status_code: &200
	};

    diesel::insert_into(apis::table)
        .values(&new_api)
        .execute(&conn)
        .expect("Error saving new post");

	HttpResponse::Ok().body("Inserting dummy content!")
}

pub fn init_routes(config: &mut web::ServiceConfig) {
	config.service(hello);
	config.service(insert);
}