use std::{
    env::{set_var, var_os},
    io::Result,
};

use actix_web::{middleware::Logger, App, HttpServer};
use dotenv::dotenv;
use routes::home_routes;

mod routes;
mod utils;

#[actix_web::main]
async fn main() -> Result<()> {
    if var_os("RUST_LOG").is_none() {
        set_var("RUST_LOG", "actix_web=info");
    }

    dotenv().ok();
    env_logger::init();

    let address = (utils::constants::ADDRESS).clone();
    let port = (utils::constants::PORT).clone();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(home_routes::config)
    })
    .bind((address, port))?
    .run()
    .await
}
