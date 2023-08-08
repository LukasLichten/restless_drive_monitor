mod api;
pub mod smart;

use actix_web::{HttpServer, App, middleware::Logger};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");

    HttpServer::new(move || {
        let logger = Logger::default();


        App::new()
        .wrap(logger)
        .service(api::get_ping)
        .service(api::get_drive_list)
    })
    .bind(("0.0.0.0", 30603))?
    .run()
    .await
}
