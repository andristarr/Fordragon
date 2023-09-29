use crate::common::database_handler::DatabaseHandler;
use crate::common::model::Item;

use futures::executor::block_on;

pub trait Validateable<T> {
    fn name(&self) -> String;
    fn validate_add(&self, dh: &DatabaseHandler) -> bool;
}

impl Validateable<Item> for Item {
    fn name(&self) -> String {
        (&self.name).to_string()
    }

    fn validate_add(&self, dh: &DatabaseHandler) -> bool {
        if let Ok(Some(_)) = block_on(dh.get::<Item>(&self.name)) {
            return true;
        }

        false
    }
}