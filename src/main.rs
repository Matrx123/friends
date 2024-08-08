use std::{
    env::{set_var, var_os},
    io::Result,
};

use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder};
use dotenv::dotenv;

#[get("/health/{name}")]
async fn health(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}")
}

#[actix_web::main]
async fn main() -> Result<()> {
    if var_os("RUST_LOG").is_none() {
        set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();
    env_logger::init();

    HttpServer::new(|| App::new().wrap(Logger::default()).service(health))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
