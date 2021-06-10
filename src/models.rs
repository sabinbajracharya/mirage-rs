use actix_web::{web};
use diesel::prelude::*;
// use diesel::debug_query;
// use diesel::*;
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

#[derive(Serialize, Deserialize)]
pub struct ContentWithActiveState {
    pub id: i32,
    pub pid_endpoint: i32,
    pub body: String,
    pub status_code: i32,
    pub request_method: String,
    pub active: bool,
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

    pub async fn find_all_from_endpoint_id(arg_id: i32) -> Result<Vec<ContentWithActiveState>, CustomError> {
        use schema::contents::dsl::*;

        let conn = connection().unwrap();

        let results = web::block(move || {
            contents
            .left_join(allows::table)
            .filter(pid_endpoint.eq(arg_id))
            .load::<(Content, Option<Allow>)>(&conn)
        }).await?;

        let results: Vec<ContentWithActiveState> = results
            .into_iter()
            .map(|(item_content, item_allow)| ContentWithActiveState {
                id              : item_content.id,
                pid_endpoint    : item_content.pid_endpoint,
                body            : item_content.body,
                status_code     : item_content.status_code,
                request_method  : item_content.request_method,
                active          : !item_allow.is_none()
            })
            .collect();

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
        let result = web::block(move || {
            // The current of Diesel doesn't yet support upsert for SQLite
            // The feature is implemented but not yet merged with stable release
            // https://github.com/diesel-rs/diesel/issues/1854

            // diesel::update(allows.find())
            //     .values(&newAllow)
            //     .on_conflict(id)
            //     .do_update()
            //     .set(&newAllow)
            //     .execute(&conn)

            use schema::allows::dsl::*;

            let query_result = diesel::insert_into(allows)
                .values(&newAllow)
                .execute(&conn);

            match query_result {
                Ok(value) => Ok("1 row inserted".to_string()),
                Err(error) => {
                    diesel::update(allows)
                        .filter(pid_endpoint.eq(newAllow.pid_endpoint))
                        .filter(request_method.eq(newAllow.request_method))
                        .set((status_code.eq(newAllow.status_code), pid_content.eq(newAllow.pid_content)))
                        .execute(&conn)?;
                    Ok("1 row updated".to_string())
                }
            }
        }).await?;

        Ok(result)
        // use schema::allows::dsl::*;
        // match xxx {
        //     Ok(s) => Ok(s),
        //     Err(error) => Err(CustomError::from(error))
        // }

        // let target = allows
        //     .filter(pid_endpoint.eq(newAllow.pid_endpoint))
        //     .filter(request_method.eq(newAllow.request_method));
        // let query = diesel::update(target)
        //                 .set(flag.eq(1));


        // let debug = debug_query::<diesel::sqlite::Sqlite, _>(&query);
        // println!("Debug Query: {}", debug);

        // Ok("1 row inserted!".to_string())
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