mod bootstrap;
mod node;
mod prime;
mod gossip;
mod crypto;
mod error;

use actix_web::{web, App, HttpServer};
use log::info;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    env_logger::init();
    info!("Starting Zynthrak...");

    // Start the bootstrap server
    HttpServer::new(|| {
        App::new()
            .service(web::scope("/api")
                .service(bootstrap::register)
                .service(bootstrap::get_update))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
