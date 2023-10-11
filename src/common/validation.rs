use crate::common::database_handler::DatabaseHandler;
use crate::common::model::Item;

use futures::executor::block_on;

pub trait Validateable<T> {
    fn uuid(&self) -> String;
    fn validate_add(&self, dh: &DatabaseHandler) -> bool;
}

impl Validateable<Item> for Item {
    fn uuid(&self) -> String {
        (&self.uuid).to_string()
    }

    fn validate_add(&self, dh: &DatabaseHandler) -> bool {
        if let Ok(Some(_)) = block_on(dh.get::<Item>(&self.uuid)) {
            return true;
        }

        false
    }
}