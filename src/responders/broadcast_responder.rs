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

    // STEP 1: Take snapshot of all (id, tx) pairs
    let snapshot = match clients.lock() {
        Ok(map) => map
            .iter()
            .map(|(id, tx)| (id.clone(), tx.clone()))
            .collect::<Vec<_>>(),
        Err(_) => return HttpResponse::InternalServerError().body("Failed to lock clients"),
    };

    // STEP 2: Loop through snapshot outside lock
    for (id, tx) in snapshot {
        if tx.is_closed() {
            // Lock only briefly to remove closed client
            let mut map = clients.lock().unwrap();
            map.remove(&id);
            continue;
        }

        let _ = tx.send(json_msg.clone());
    }

    HttpResponse::Ok().body("Broadcast message sent")
}
