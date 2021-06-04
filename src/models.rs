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
    pub pid: &'a i32,
    pub body: &'a str,
    pub status_code: &'a i32,
}