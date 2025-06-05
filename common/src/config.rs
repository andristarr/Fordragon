use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::{env, fs};

#[derive(Serialize, Deserialize, Debug)]
struct ConfigString {
    pub db_uri: String,
    pub db_name: String,
    pub tick_count: u8,
    pub log_level: String,
}

#[derive(Debug)]
pub struct Config {
    pub db_uri: String,
    pub db_name: String,
    pub tick_count: u8,
    pub log_level: log::LevelFilter,
}

impl Config {
    pub fn get() -> Result<Config> {
        Ok(serde_json::from_str::<ConfigString>(&fs::read_to_string("server.json")?)?.into())
    }
}

impl From<ConfigString> for Config {
    fn from(cfg: ConfigString) -> Self {
        Config {
            db_uri: cfg.db_uri,
            db_name: cfg.db_name,
            tick_count: cfg.tick_count,
            log_level: cfg.log_level.parse().unwrap_or(log::LevelFilter::Info),
        }
    }
}
