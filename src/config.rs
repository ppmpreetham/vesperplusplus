use std::sync::OnceLock;

#[derive(Debug)]
pub struct Config {
    pub username: String,
}

pub static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init_config(user: String) {
    let trimmed = user.trim().to_string();
    CONFIG.get_or_init(|| Config { username: trimmed });
}

pub fn get_username() -> &'static str {
    &CONFIG.get().expect("username must be provided").username
}
