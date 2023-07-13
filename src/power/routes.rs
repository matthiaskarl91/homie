use actix_web::{Responder, get, web, post, error, HttpResponse};
use crate::power::service::*;

use crate::DbPool;

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
async fn add_power(pool: web::Data<DbPool>) -> impl Responder {
    let power = web::block(move || {
        let mut conn = pool.get().expect("couldnt get db connection from pool");
        
        insert_new_power(&mut conn, 30)
    }).await;

    HttpResponse::Ok().json("ok")
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/power").service(get_all).service(add_power)
    );

}
