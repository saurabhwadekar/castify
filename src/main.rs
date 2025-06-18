use actix_web::{App, HttpServer, web};
use responders::broadcast_responder::broadcast;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use ws::types::Clients;
pub mod responders;
pub mod utils;
use responders::status_responder::status_responder;
pub mod config;
pub mod ws;
use config::PORT;
use ws::handler::handler;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));
    let count: Arc<Mutex<i64>> = Arc::new(Mutex::new(0));
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(clients.clone()))
            .app_data(web::Data::new(count.clone()))
            .service(status_responder)
            .service(handler)
            .service(broadcast)
    })
    .bind(("0.0.0.0", *PORT))?
    .run()
    .await
}
