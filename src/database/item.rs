use super::Database;
use super::error::{Error, Kind};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub min_stock: Option<i32>,
    pub max_stock: Option<i32>,
    pub stock: i64,
}

impl Database {
    pub async fn get_item(
        &self,
        item_id: i32,
    ) -> Result<Item, Error> {
        let connection = self.pool.get().await?;

        let row_option = connection
            .query_opt(
                &connection.prepare_cached(
                    r#"
                        SELECT i.name, i.min_stock, i.max_stock, t.stock
                        FROM stock_item i 
                        INNER JOIN stock_total t
                            ON i.id = t.item_id
                        WHERE i.id = $1
                    "#
                ).await?,
                &[&item_id]
            ).await
            .map_err(|e| Error(
                Kind::Query,
                "Error getting item".to_owned(),
                Some(Box::new(e))
            ))?;
        
        if row_option.is_none() {
            return Err(Error(
                Kind::ItemNotFound,
                "".to_owned(),
                None
            ));
        }
        
        let row = row_option.unwrap();

        Ok(Item {
            id: item_id,
            name: row.get("name"),
            min_stock: row.get("min_stock"),
            max_stock: row.get("max_stock"),
            stock: row.get("stock"),
        })
    }
}