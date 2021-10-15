use super::error::ServiceError;
use crate::{
    database::Database,
    dtos::item_dtos::{Item, NewItemRequest},
};

#[derive(Clone)]
pub struct ItemService {
    database: Database,
}

impl ItemService {
    pub fn new(database: Database) -> ItemService {
        ItemService { database: database }
    }

    pub async fn get_items(&self) -> Result<Vec<Item>, ServiceError> {
        self.database.get_items().await.map_err(|err| err.into())
    }

    pub async fn get_item(&self, item_id: i32) -> Result<Item, ServiceError> {
        self.database
            .get_item(item_id)
            .await
            .map_err(|err| err.into())
    }

    pub async fn create_item(&self, item: NewItemRequest) -> Result<Item, ServiceError> {
        self.database
            .create_item(item)
            .await
            .map_err(|err| err.into())
    }
}
