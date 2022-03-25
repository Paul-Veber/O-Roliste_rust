use actix_web::web;

use crate::controllers::base::{greet, index};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/").route(web::get().to(index)));
    cfg.service(web::resource("/test/{name}").route(web::get().to(greet)));
}
