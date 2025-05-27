use crate::config::{GLOBAL_TOKEN, SERVER_SECRET, USE_GLOBAL_TOKEN, VERIFICATION_URL};
use reqwest;

pub async fn verify_token(token: &str) -> bool {
    if *USE_GLOBAL_TOKEN {
        return token == *GLOBAL_TOKEN;
    }

    let client = reqwest::Client::new();

    match client
        .post(VERIFICATION_URL.as_str())
        .json(&serde_json::json!({ "secret":*SERVER_SECRET,"token": token }))
        .send()
        .await
    {
        Ok(resp) if resp.status().is_success() => true,
        _ => false,
    }
}
