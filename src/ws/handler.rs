use crate::utils::verify_token::verify_token;
use crate::ws::types::Clients;
use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_ws::{Message, handle};
use futures_util::StreamExt as _;
use tokio::sync::mpsc;
use uuid;

/// Helper function to safely remove a client from the shared Clients map
fn cleanup_client(clients: &Clients, client_id: &str) {
    let mut locked = clients.lock().unwrap();
    let removed = locked.remove(client_id);
    // Uncomment the following lines for debugging purposes
    #[cfg(debug_assertions)]
    {
        if removed.is_some() {
            println!("Client {} cleaned up and removed", client_id);
        } else {
            println!("Client {} was already removed or not found", client_id);
        }
    }
}

#[actix_web::get("/ws")]
pub async fn handler(
    req: HttpRequest,
    stream: web::Payload,
    clients: web::Data<Clients>,
) -> Result<HttpResponse, Error> {
    let query = req.query_string();

    // Check for the presence of a token in the query string
    if query.contains("token") {
        let token: Vec<&str> = query.split("token=").collect();
        if token.len() > 1 {
            let token_is_valid = verify_token(token[1]).await;
            if !token_is_valid {
                return Ok(HttpResponse::Unauthorized().body("Invalid token"));
            }
        } else {
            return Ok(HttpResponse::BadRequest().body("Token is missing"));
        }
    } else {
        return Ok(HttpResponse::BadRequest().body("Token is required"));
    }

    let (res, mut session, mut msg_stream) = handle(&req, stream)?;
    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    let client_id = uuid::Uuid::new_v4().to_string();
    clients.lock().unwrap().insert(client_id.clone(), tx);
    let clients_clone = clients.clone();
    // Receiver → listen incoming client messages
    actix_web::rt::spawn({
        let client_id = client_id.clone();
        let mut session = session.clone();
        async move {
            while let Some(msg_result) = msg_stream.next().await {
                match &msg_result {
                    Ok(Message::Ping(bytes)) => {
                        let _ = session.pong(bytes).await;
                    }
                    Err(_) => {
                        break;
                    }
                    _ => {}
                }

                #[cfg(debug_assertions)]
                {
                    match msg_result {
                        Ok(Message::Text(_text)) => {
                            println!("Received from client {}: {}", client_id, _text);
                        }
                        Ok(Message::Close(_)) => {
                            println!("Client {} requested close", client_id);
                        }
                        Ok(Message::Pong(_)) => {
                            println!("Received pong response from client {}", client_id);
                        }
                        Err(_e) => {
                            println!("Error on client {}: {:?}", client_id, _e);
                        }
                        _ => {}
                    }
                }
            }

            cleanup_client(&clients_clone, &client_id);
        }
    });

    // Sender → push server messages to client
    actix_web::rt::spawn({
        let clients_clone = clients.clone();
        let client_id = client_id.clone();
        let mut session = session.clone();
        async move {
            while let Some(server_msg) = rx.recv().await {
                if session.text(server_msg).await.is_err() {
                    break;
                }
            }
            cleanup_client(&clients_clone, &client_id);
        }
    });

    // send ping message to client every 60 seconds
    actix_web::rt::spawn({
        let clients_clone = clients.clone();
        let client_id = client_id.clone();
        async move {
            loop {
                tokio::time::sleep(std::time::Duration::from_secs(60)).await;
                if session.ping(b"ping").await.is_err() {
                    break;
                }
            }
            cleanup_client(&clients_clone, &client_id);
        }
    });

    Ok(res)
}
