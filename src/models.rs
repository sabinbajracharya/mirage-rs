use actix_web::{web};
use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

use crate::error_handler::CustomError;
use crate::schema;
use crate::db::connection;
use super::schema::{apis, endpoints, contents, allows};

#[derive(Queryable)]
pub struct Api {
    pub id: i32,
    pub pid: i32,
    pub body: String,
    pub status_code: i32,
}

#[derive(Insertable)]
#[table_name = "apis"]
pub struct NewApi<'a> {
    pub pid: i32,
    pub body: &'a str,
    pub status_code: i32,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Endpoint {
    pub id: i32,
    pub path: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "endpoints"]
pub struct NewEndpoint {
    pub path: String,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct Content {
    pub id: i32,
    pub pid_endpoint: i32,
    pub body: String,
    pub status_code: i32,
    pub request_method: String,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "contents"]
pub struct NewContent {
    pub pid_endpoint: i32,
    pub body: String,
    pub status_code: i32,
    pub request_method: String,
}

impl Endpoint {
    pub async fn create(endpoint: NewEndpoint) -> Result<String, CustomError> {

        let conn = connection().unwrap();
        let endpoint = NewEndpoint::from(endpoint);

        let result = web::block(move || {
            diesel::insert_into(endpoints::table)
                .values(&endpoint)
                .execute(&conn)
        }).await?;

        Ok(format!("Inserted {} row in the db!", result))
    }

    pub async fn find_all() -> Result<Vec<Endpoint>, CustomError> {
        use schema::endpoints::dsl::*;

        let conn = connection().unwrap();

        let results = web::block(move || {
            endpoints
                .load::<Endpoint>(&conn)
        }).await?;

        Ok(results)
    }
}

impl NewEndpoint {
    fn from (endpoint: NewEndpoint) -> NewEndpoint {
        NewEndpoint {
            path: endpoint.path
        }
    }
}

impl Content {
    pub async fn create(content: NewContent) -> Result<String, CustomError> {
        let conn = connection().unwrap();
        let new_content = NewContent::from(content);

        web::block(move || {
            diesel::insert_into(contents::table)
                .values(&new_content)
                .execute(&conn)
        }).await?;

        Ok(String::from("Success!"))
    }

    pub async fn find_all() -> Result<Vec<Content>, CustomError> {
        use schema::contents::dsl::*;

        let conn = connection().unwrap();

        let results = web::block(move || {
            contents.load::<Content>(&conn)
        }).await?;

        Ok(results)
    }
}

impl NewContent {
    fn from(content: NewContent) -> NewContent {
        NewContent {
            ..content
        }
    }
}