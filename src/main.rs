use actix_web::{web, App, HttpServer};
use tera::Tera;
use dotenv::dotenv;
use std::env;


mod controllers;
mod router;
mod utils;

use utils::error_handling::error_handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env::set_var("RUST_LOG", "actix_web=debug");
    let host = env::var("HOST").expect("Host not set");
    let port = env::var("PORT").expect("Port not set");

    HttpServer::new(move|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
            .app_data(web::Data::new(tera))
            .configure(router::config)
            .service(web::scope("").wrap(error_handlers()))
    })
     .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
