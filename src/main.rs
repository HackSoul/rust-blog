mod api;
use actix_web::{middleware, web, App, HttpServer};

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .service(web::resource("/index.html").to(|| "Hello world!"))
            .service(web::resource("/").to(api::index::index))
    }).bind("127.0.0.1:8080")?.run()
}