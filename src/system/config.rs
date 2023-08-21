use serde::Deserialize;
use figment::{Figment, providers::{Format, Toml, Env}};

#[derive(Deserialize, Clone, Debug)]
pub struct Mongo {
    pub host: String,
    pub database_name: String
}

#[derive(Deserialize, Clone, Debug)]
pub struct Config {
    pub port: u16,
    pub mongo: Option<Mongo>
}

pub fn get_config() -> Config {
    return Figment::new()
        .merge(Toml::file("config.toml"))
        .merge(Env::prefixed("ONCALLS_"))
        .extract().unwrap();
}