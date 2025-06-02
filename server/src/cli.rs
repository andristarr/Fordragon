mod common;

use std::env;

use common::config::Config;
use common::error::Error;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let command = args
        .get(1)
        .ok_or(Error::InvalidArguments("No command received".to_string()))
        .unwrap();

    let config = Config::get().unwrap();
}
