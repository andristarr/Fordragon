mod common;

use std::env;
use serde_json::from_str;

use common::model::{Item};
use common::database_handler::DatabaseHandler;
use common::config::Config;
use common::error::Error;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    let command = args.get(1).ok_or(Error::InvalidArguments("No command received".to_string())).unwrap();

    let config = Config::get().unwrap();

    match command.as_str() {
        "add" => {
            let added_type = args.get(2).expect("No type received");

            match added_type.as_str() {
                "item" => {
                    let item = args.get(3).expect("No item received");

                    let item = from_str::<Item>(item).expect("Item cannot be parsed");

                    let db = DatabaseHandler::new(&config.db_uri).await.unwrap().connect_database(&config.db_name).await.connect_collection("items").unwrap();

                    match db.add(item).await {
                        Err(err) => {println!("Error occured adding: {:?}", err)},
                        _ => {}
                    }
                }
                _ => {}
            }
        },
        "remove" => {
            let removed_type = args.get(2).expect("No type received");

            match removed_type.as_str() {
                "item" => {
                    let item_name = args.get(3).expect("No item name received");

                    let db = DatabaseHandler::new(&config.db_uri).await.unwrap().connect_database(&config.db_name).await.connect_collection("items").unwrap();

                    db.remove::<Item>(item_name).await;
                }
                _ => {}
            }
        }
        _ => {}
    }
    
    println!("{:?}", args);
}