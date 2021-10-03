use super::error::{DatabaseError, Kind};
use super::Database;
use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemInformation {
    pub id: i32,
    pub name: String,
    pub min_stock: Option<i32>,
    pub max_stock: Option<i32>,
    pub stock: i32,
}

impl Database {
    pub async fn get_items(&self) -> Result<Vec<ItemInformation>, DatabaseError> {
        let connection = self.pool.get().await?;

        let rows = connection
            .query(
                &connection
                    .prepare_cached(
                        r#"
                            SELECT id, name, min_stock, max_stock, stock
                            FROM stock.item
                        "#,
                    )
                    .await?,
                &[],
            )
            .await
            .map_err(|e| {
                DatabaseError(
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

    pub async fn get_item(&self, item_id: i32) -> Result<ItemInformation, DatabaseError> {
        let connection = self.pool.get().await?;

        let row_option = connection
            .query_opt(
                &connection
                    .prepare_cached(
                        r#"
                            SELECT name, min_stock, max_stock, stock
                            FROM stock.item
                            WHERE id = $1
                        "#,
                    )
                    .await?,
                &[&item_id],
            )
            .await
            .map_err(|e| {
                DatabaseError(
                    Kind::Query,
                    "Error getting item".to_owned(),
                    Some(Box::new(e)),
                )
            })?;

        if row_option.is_none() {
            return Err(DatabaseError(Kind::ItemNotFound, "".to_owned(), None));
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
