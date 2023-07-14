use actix_web::{Responder, HttpResponse, get, post, HttpServer, App, web};
use diesel::SqliteConnection;

pub mod schema;
mod power;
mod source;

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")

}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub type DbPool = r2d2::Pool<diesel::r2d2::ConnectionManager<SqliteConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = diesel::r2d2::ConnectionManager::<SqliteConnection>::new("homie.db");
    let pool = r2d2::Pool::builder().build(manager).expect("database URL should be valid path to SQLite DB file");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::scope("/api")
                     .configure(power::routes::config)
                     .configure(source::routes::config)
            )
            .service(hello)
            .service(echo)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
