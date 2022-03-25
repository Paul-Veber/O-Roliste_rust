use actix_web::{ App, HttpServer, web};
use tera::Tera;

mod router;
mod controllers;
mod utils;

use utils::error_handling::error_handlers;

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

        App::new()
        .app_data(web::Data::new(tera))
        .configure(router::config)
        .service(web::scope("").wrap(error_handlers()))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}