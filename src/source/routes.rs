use actix_web::{get, post, web, Responder, error, HttpResponse};
use serde::Deserialize;

use crate::DbPool;

use super::service::{insert_new_source, get_sources};

#[derive(Deserialize)]
struct SourceForm {
    name: String,
    unit: String,
}

#[post("/")]
async fn insert_source(pool: web::Data<DbPool>, info: web::Json<SourceForm>) -> actix_web::Result<impl Responder> {
    web::block(move || {
        let mut conn = pool.get().expect("Error getting connection");
        
        insert_new_source(&mut conn, &info.name, &info.unit)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json("ok"))
}

#[get("/")]
async fn get_all(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let data = web::block(move || {
        let mut conn = pool.get().expect("Error getting connection");

        get_sources(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(data))

}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/source").service(get_all).service(insert_source)
    );

}
