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

#[derive(Serialize, Deserialize, Queryable)]
pub struct Allow {
    pub id: i32,
    pub pid_endpoint: i32,
    pub pid_content: i32,
    pub status_code: i32,
    pub request_method: String,
    pub flag: i32,
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "allows"]
pub struct NewAllow {
    pub pid_endpoint: i32,
    pub pid_content: i32,
    pub status_code: i32,
    pub request_method: String,
    #[serde(default)]
    pub flag: i32,
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

        Ok(String::from("1 row inserted!"))
    }

    pub async fn find_all() -> Result<Vec<Content>, CustomError> {
        use schema::contents::dsl::*;

        let conn = connection().unwrap();

        let results = web::block(move || {
            contents.load::<Content>(&conn)
        }).await?;

        Ok(results)
    }

    pub async fn find_all_from_endpoint_id(arg_id: i32) -> Result<Vec<Content>, CustomError> {
        use schema::contents::dsl::*;

        let conn = connection().unwrap();

        let results = web::block(move || {
            contents
            .filter(pid_endpoint.eq(arg_id))
            .load::<Content>(&conn)
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

impl Allow {
    pub async fn create(allow: NewAllow) -> Result<String, CustomError> {
        let conn = connection().unwrap();

        let newAllow = NewAllow::from(allow);
        web::block(move || {
            diesel::insert_into(allows::table)
                .values(&newAllow)
                .execute(&conn)
        }).await?;

        Ok("1 row inserted!".to_string())
    }

    pub async fn find_all() -> Result<Vec<Allow>, CustomError> {
        use schema::allows::dsl::*;

        let conn = connection().unwrap();

        let results = web::block(move || {
            allows.load::<Allow>(&conn)
        }).await?;

        Ok(results)
    }
}

impl NewAllow {
    fn from(allow: NewAllow) -> NewAllow {
        NewAllow {
            flag: 1,
            ..allow
        }
    }
}