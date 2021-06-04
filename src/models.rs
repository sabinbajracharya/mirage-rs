use diesel::prelude::*;
use diesel::result::Error;
use serde::{Deserialize, Serialize};

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

impl Endpoint {
    pub fn create(endpoint: NewEndpoint) -> Result<&'static str, Error> {

        let conn = connection().unwrap();
        let endpoint = NewEndpoint::from(endpoint);

        diesel::insert_into(endpoints::table)
            .values(&endpoint)
            .execute(&conn)?;

        Ok("Inserted 1 row.")
    }

    pub fn find_all() -> Result<Vec<Endpoint>, Error> {
        use schema::endpoints::dsl::*;

        let conn = connection().unwrap();

        let results = endpoints
            .load::<Endpoint>(&conn)
            .expect("Error loading endpoints");

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

