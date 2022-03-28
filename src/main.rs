#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpServer};
use diesel::{r2d2::{self, ConnectionManager}, PgConnection};
use dotenv::dotenv;
use std::env;
use tera::Tera;

mod controllers;
mod database;
mod router;
mod utils;

use utils::error_handling::error_handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");

    // set up database connection pool
    let conn_spec = std::env::var("DATABASE_URL").expect("DATABASE_URL");
    let manager = ConnectionManager::<PgConnection>::new(conn_spec);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    log::info!("starting HTTP server");

    HttpServer::new(move || {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            // set up DB pool to be used with web::Data<Pool> extractor
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tera))
            .configure(router::config)
            .service(web::scope("").wrap(error_handlers()))
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
