use crate::config::SERVER_SECRET;
use crate::ws::types::Clients;
use actix_web::{HttpResponse, web};
use serde_json::Value;
use std::collections::HashMap;

#[actix_web::post("/broadcast")]
pub async fn broadcast(
    clients: web::Data<Clients>,
    web::Json(payload): web::Json<HashMap<String, Value>>,
) -> HttpResponse {
    let token = payload.get("token").cloned().unwrap_or(Value::Null);
    if token != *SERVER_SECRET {
        return HttpResponse::Unauthorized().body("Invalid token");
    }
    let message = payload.get("message").cloned().unwrap_or(Value::Null);

    let json_msg = serde_json::json!({
        "message": message
    })
    .to_string();

    // Take snapshot of all tx
    let snapshot = match clients.lock() {
        Ok(map) => map.iter().map(|(_id, tx)| tx.clone()).collect::<Vec<_>>(),
        Err(_) => return HttpResponse::InternalServerError().body("Failed to lock clients"),
    };

    // Loop through snapshot outside lock
    for tx in snapshot {
        if tx.is_closed() {
            continue;
        }
        let _ = tx.send(json_msg.clone());
    }

    HttpResponse::Ok().body("Broadcast message sent")
}
