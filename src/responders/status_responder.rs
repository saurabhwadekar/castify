use actix_web::{HttpResponse, Responder};

#[actix_web::get("/")]
pub async fn status_responder() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/plain")
        .body("Running...")
}
