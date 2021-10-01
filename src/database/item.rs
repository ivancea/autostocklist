use super::error::{Error, Kind};
use super::Database;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemInformation {
    pub id: i32,
    pub name: String,
    pub min_stock: Option<i32>,
    pub max_stock: Option<i32>,
    pub stock: i64,
}

impl Database {
    pub async fn get_items(&self) -> Result<Vec<ItemInformation>, Error> {
        let connection = self.pool.get().await?;

        let rows = connection
            .query(
                &connection
                    .prepare_cached(
                        r#"
                        SELECT i.id, i.name, i.min_stock, i.max_stock, t.stock
                        FROM stock_item i 
                        INNER JOIN stock_total t
                            ON i.id = t.item_id
                    "#,
                    )
                    .await?,
                &[],
            )
            .await
            .map_err(|e| {
                Error(
                    Kind::Query,
                    "Error getting item".to_owned(),
                    Some(Box::new(e)),
                )
            })?;

        Ok(rows
            .iter()
            .map(|row| ItemInformation {
                id: row.get("id"),
                name: row.get("name"),
                min_stock: row.get("min_stock"),
                max_stock: row.get("max_stock"),
                stock: row.get("stock"),
            })
            .collect())
    }

    pub async fn get_item(&self, item_id: i32) -> Result<ItemInformation, Error> {
        let connection = self.pool.get().await?;

        let row_option = connection
            .query_opt(
                &connection
                    .prepare_cached(
                        r#"
                        SELECT i.name, i.min_stock, i.max_stock, t.stock
                        FROM stock_item i 
                        INNER JOIN stock_total t
                            ON i.id = t.item_id
                        WHERE i.id = $1
                    "#,
                    )
                    .await?,
                &[&item_id],
            )
            .await
            .map_err(|e| {
                Error(
                    Kind::Query,
                    "Error getting item".to_owned(),
                    Some(Box::new(e)),
                )
            })?;

        if row_option.is_none() {
            return Err(Error(Kind::ItemNotFound, "".to_owned(), None));
        }

        let row = row_option.unwrap();

        Ok(ItemInformation {
            id: item_id,
            name: row.get("name"),
            min_stock: row.get("min_stock"),
            max_stock: row.get("max_stock"),
            stock: row.get("stock"),
        })
    }
}
