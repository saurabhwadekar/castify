use super::get_env_vars::get_env_var;
use reqwest;

pub async fn verify_token(token: &str) -> bool {
    let client = reqwest::Client::new();
    match client
        .post(get_env_var(
            "VERIFICATION_URL",
            "http://localhost:3000/verify".to_string(),
        ))
        .json(&serde_json::json!({ "secret":get_env_var("SERVER_SECRET", "SERVER_SECRET_KEY".to_string()),"token": token }))
        .send()
        .await
    {
        Ok(resp) if resp.status().is_success() => true,
        _ => false,
    }
}
