use super::get_env_vars::get_env_var;
use reqwest;

pub async fn verify_token(token: &str) -> bool {
    let g_token = get_env_var("GLOBAL_TOKEN", "GLOBAL_TOKEN".to_string());

    if get_env_var("USE_GLOBAL_TOKEN", false) {
        return token == g_token;
    }

    let client = reqwest::Client::new();
    let secret = get_env_var("SERVER_SECRET", "SERVER_SECRET_KEY".to_string());
    let verification_url = get_env_var(
        "VERIFICATION_URL",
        "http://localhost:3000/verify".to_string(),
    );

    match client
        .post(verification_url)
        .json(&serde_json::json!({ "secret":secret,"token": token }))
        .send()
        .await
    {
        Ok(resp) if resp.status().is_success() => true,
        _ => false,
    }
}
