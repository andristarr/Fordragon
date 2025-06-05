use async_trait::async_trait;

use crate::{database_handler::DatabaseHandler, model::Item};

#[async_trait]
pub trait Validateable<T> {
    fn uuid(&self) -> String;
    async fn validate_add(&self, dh: &DatabaseHandler) -> bool;
}

#[async_trait]
impl Validateable<Item> for Item {
    fn uuid(&self) -> String {
        self.uuid.to_string()
    }

    async fn validate_add(&self, dh: &DatabaseHandler) -> bool {
        if let Ok(Some(_)) = dh.get::<Item>(&self.uuid).await {
            return true;
        }

        false
    }
}
