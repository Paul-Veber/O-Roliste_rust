use actix_web::{error, web, Error, HttpResponse, Responder};
use diesel::associations::HasTable;
use diesel::{
    pg::Pg,
    r2d2::{ConnectionManager, Pool},
};
use diesel::{r2d2, PgConnection, RunQueryDsl};
use std::{collections::HashMap, sync::Mutex};

use crate::database::models::User;

pub async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!")
}

/* pub async fn index(tmpl: web::Data<tera::Tera>, query: web::Query<HashMap<String, String>>) -> Result<HttpResponse, Error> {
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
 */
type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn index(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    use crate::database::schema::users::dsl::*;

    let all_users = web::block(move || {
        let connexion = pool.get().expect("couldn't get db connection from pool");
        users.load::<User>(&connexion).expect("error user database")
    })
    .await
    .map_err(actix_web::error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(all_users))
}
