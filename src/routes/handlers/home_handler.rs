use actix_web::{get, web, Responder};

#[get("/health/{name}")]
async fn check_health(name: web::Path<String>) -> impl Responder {
    format!("hello {name}")
}

#[get("/test")]
async fn test() -> impl Responder {
    format!("Hello RUST!!")
}
