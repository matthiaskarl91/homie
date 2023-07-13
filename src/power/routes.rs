use actix_web::{Responder, get, web, post, error, HttpResponse};
use chrono::{NaiveDateTime, Utc};
use serde::Deserialize;
use crate::power::service::*;

use crate::DbPool;

#[derive(Deserialize)]
struct PowerForm {
    value: i32,
    timestamp: Option<NaiveDateTime>,
}

#[get("/")]
async fn get_all(pool: web::Data<DbPool>) -> actix_web::Result<impl Responder> {
    let data = web::block(move || {
        let mut conn = pool.get().expect("couldnt get db connection from pool");

        get_power(&mut conn)
    })
    .await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(data))
}

#[post("/")]
async fn add_power(pool: web::Data<DbPool>, info: web::Json<PowerForm>) -> actix_web::Result<impl Responder> {
    web::block(move || {
        let mut conn = pool.get().expect("couldnt get db connection from pool");
        let timestamp = info.timestamp.unwrap_or(Utc::now().naive_utc());
        insert_new_power(&mut conn, info.value, timestamp)
    }).await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json("ok"))
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/power").service(get_all).service(add_power)
    );

}
