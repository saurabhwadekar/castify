use std::env;

pub fn get_env_var<T>(var_name: &str, default: T) -> T
where
    T: std::str::FromStr,
{
    env::var(var_name)
        .ok()
        .and_then(|v| v.parse::<T>().ok())
        .unwrap_or(default)
}
