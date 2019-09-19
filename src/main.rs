mod api;
mod service;
mod dao;
mod entity;

use actix_web::{middleware, web, App, HttpServer};

#[macro_use]
extern crate diesel;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/").to(api::index::index))
    }).bind("127.0.0.1:8080")?.run()
}