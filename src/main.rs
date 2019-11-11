extern crate actix;
extern crate actix_web;

use actix::prelude::*;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer, Responder};
use env_logger;

fn index(name: web::Path<String>) -> impl Responder {
    format!("Hello {}!", name)
}

fn main() {
    let host = "127.0.0.1:3000";
    let sys = System::new("rca");

    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(web::scope("/resources").route("/{name}", web::get().to(index)))
    })
    .workers(4)
    .backlog(1024)
    .bind(host)
    .unwrap()
    .start();

    println!("Started http server: {}", host);

    let _ = sys.run();
}
