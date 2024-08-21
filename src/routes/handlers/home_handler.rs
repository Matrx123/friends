use actix_web::{get, web, Responder};

use crate::utils::api_response::ApiResponse;

#[get("/health/{name}")]
async fn check_health(name: web::Path<String>) -> impl Responder {
    ApiResponse::new(200, String::from(format!("hello {name}!!")))
}

#[get("/test")]
async fn test() -> impl Responder {
    ApiResponse::new(200, String::from("Hello RUST !!"))
}
