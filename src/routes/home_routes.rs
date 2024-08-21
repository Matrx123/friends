use actix_web::web;

use super::handlers::home_handler;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/api/v1")
            .service(home_handler::check_health)
            .service(home_handler::test),
    );
}
