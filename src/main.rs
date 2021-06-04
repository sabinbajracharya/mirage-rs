use actix_web::{App, HttpServer};

mod route;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(route::init_routes)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}