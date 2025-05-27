use actix_web::{App, HttpServer};
pub mod responders;
pub mod utils;
use responders::status_responder::status_responder;
use utils::get_env_vars::get_env_var;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let port: u16 = get_env_var("PORT", 8000);

    HttpServer::new(|| App::new().service(status_responder))
        .bind(("0.0.0.0", port))?
        .run()
        .await
}
