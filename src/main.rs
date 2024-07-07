#[macro_use]
extern crate diesel;

use actix_web::{App, HttpServer};
use dotenv::dotenv;

mod models;
mod schema;
mod handlers;
mod routes;
mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let pool = config::database::establish_connection();

    HttpServer::new(move || {
        App::new()
            .configure(|cfg| routes::api::init(cfg, pool.clone()))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
