use actix_web::{web, error, Responder, Error, HttpResponse};
use std::collections::HashMap;
pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

pub async fn index(tmpl: web::Data<tera::Tera>, query: web::Query<HashMap<String, String>>) -> Result<HttpResponse, Error> {
    let s = if let Some(name) = query.get("name") {
        // submitted form
        let mut ctx = tera::Context::new();
        ctx.insert("name", name);
        ctx.insert("text", "Welcome!");
        tmpl.render("pages/user.jinja", &ctx)
            .map_err(|_| error::ErrorInternalServerError("Template error"))?
    } else {
        tmpl.render("pages/index.jinja", &tera::Context::new())
            .map_err(|_| error::ErrorInternalServerError("Template error"))?
    };
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}
