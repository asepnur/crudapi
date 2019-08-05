#[macro_use]
extern crate diesel;
extern crate crudapi;

#[macro_use]
extern crate serde_derive;
use actix_web::{middleware, web, App, HttpServer,  HttpRequest, HttpResponse};
use futures::{Future, Stream};
use diesel::prelude::*;

use diesel::r2d2::{self, ConnectionManager};

use dotenv::dotenv;
use std::env;

pub mod posts;

fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool: posts::models::Pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");


    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/").route(web::post().to(show_post)))
    })
    .bind("127.0.0.1:8080")?
    .run()

}

fn show_post(
    pool: web::Data<posts::models::Pool>,
) -> HttpResponse {
    let result = posts::show_post(pool);
    HttpResponse::Ok().json(result) // <- send response
}