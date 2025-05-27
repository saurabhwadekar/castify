use actix_web::{App, HttpServer};
pub mod responders;
pub mod utils;
use responders::status_responder::status_responder;
pub mod config;
use config::PORT;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    HttpServer::new(|| App::new().service(status_responder))
        .bind(("0.0.0.0", *PORT))?
        .run()
        .await
}
