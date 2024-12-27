use config::{Config, Environment};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct AppConfig {
    pub port: u16,
    pub database_url: String
}

pub fn load_config() -> AppConfig {
    let config = Config::builder()
        .add_source(Environment::default())
        .build()
        .expect("cannot build config");

    config.try_deserialize().expect("cannot deserialize config")
}
