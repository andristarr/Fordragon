use std::{env, fs};
use serde::{Deserialize, Serialize};
use crate::common::error::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub db_uri: String,
    pub db_name: String
}

impl Config {
    pub fn get() -> Result<Config, Error> {
        let current_dir = env::current_dir()?;

        println!("{:?}", current_dir);

        Ok(serde_json::from_str::<Config>(&fs::read_to_string("server.json")?)?)
    }
}