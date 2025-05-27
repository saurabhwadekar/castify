use crate::utils::get_env_vars::get_env_var;
use once_cell::sync::Lazy;

pub static PORT: Lazy<u16> = Lazy::new(|| get_env_var("PORT", 8000));
pub static USE_GLOBAL_TOKEN: Lazy<bool> = Lazy::new(|| get_env_var("USE_GLOBAL_TOKEN", false));
pub static GLOBAL_TOKEN: Lazy<String> =
    Lazy::new(|| get_env_var("GLOBAL_TOKEN", String::from("GLOBAL_TOKEN")));
pub static VERIFICATION_URL: Lazy<String> =
    Lazy::new(|| get_env_var("VERIFICATION_URL", String::from("")));
pub static SERVER_SECRET: Lazy<String> =
    Lazy::new(|| get_env_var("SERVER_SECRET", String::from("SERVER_SECRET_KEY")));
