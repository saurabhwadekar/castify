use actix_web::{App, HttpServer};
pub mod responders;
use responders::status_responder::status_responder;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(status_responder))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
