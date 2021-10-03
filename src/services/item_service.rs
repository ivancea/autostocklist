use super::error::ServiceError;
use crate::database::{item::ItemInformation, Database};
use chrono::NaiveDate;

#[derive(Clone)]
pub struct ItemService {
    database: Database,
}

impl ItemService {
    pub fn new(database: Database) -> ItemService {
        ItemService { database: database }
    }

    pub async fn get_item(&self, item_id: i32) -> Result<ItemInformation, ServiceError> {
        self.database
            .get_item(item_id)
            .await
            .map_err(|err| err.into())
    }

    pub async fn get_items(&self) -> Result<Vec<ItemInformation>, ServiceError> {
        self.database.get_items().await.map_err(|err| err.into())
    }

    /// Updates the item stock loss on a given date.
    /// Positive values decrease the stock, negative values increase it.
    /// If the total stock or the stock loss on the given date is going to be negative, an error is returned.
    ///
    /// This function returns the new total stock on the given date.
    pub async fn update_stock_loss(
        &self,
        item_id: i32,
        date: NaiveDate,
        quantity: i32,
    ) -> Result<i32, ServiceError> {
        let item = self.database.get_item(item_id).await?;

        if item.stock - quantity < 0 {
            return Err(ServiceError::Input(format!(
                "Stock loss is higher than current stock. The current stock is {}",
                item.stock
            )));
        }

        self.database
            .update_stock_loss(item_id, date, quantity)
            .await
            .map_err(|err| err.into())
            .map(|_| item.stock - quantity)
    }

    /// Updates the item stock resupply on a given date.
    /// Positive values increases the stock, negative values decreases it.
    /// If the total stock or the stock resupply on the given date is going to be negative, an error is returned.
    ///
    /// This function returns the new total stock on the given date.
    pub async fn update_stock_resupply(
        &self,
        item_id: i32,
        date: NaiveDate,
        quantity: i32,
    ) -> Result<i32, ServiceError> {
        let item = self.database.get_item(item_id).await?;

        if item.stock + quantity < 0 {
            return Err(ServiceError::Input(format!(
                "Stock resupply change would be reduce stock below 0. The current stock is {}",
                item.stock
            )));
        }

        self.database
            .update_stock_resupply(item_id, date, quantity)
            .await
            .map_err(|err| err.into())
            .map(|_| item.stock + quantity)
    }
}
