use crate::dtos::item_dtos::{Item, NewItemRequest};

use super::{
    error::{DatabaseError, Kind},
    Database,
};

impl Database {
    pub async fn get_items(&self) -> Result<Vec<Item>, DatabaseError> {
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
            .map(|row| Item {
                id: row.get("id"),
                name: row.get("name"),
                min_stock: row.get("min_stock"),
                max_stock: row.get("max_stock"),
                stock: row.get("stock"),
            })
            .collect())
    }

    pub async fn get_item(&self, item_id: i32) -> Result<Item, DatabaseError> {
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

        Ok(Item {
            id: item_id,
            name: row.get("name"),
            min_stock: row.get("min_stock"),
            max_stock: row.get("max_stock"),
            stock: row.get("stock"),
        })
    }

    pub async fn create_item(&self, item: NewItemRequest) -> Result<Item, DatabaseError> {
        let connection = self.pool.get().await?;

        let inserted_row_ids = connection
            .query(
                &connection
                    .prepare_cached(
                        r#"
                            INSERT INTO stock.item (name, min_stock, max_stock, stock)
                            VALUES ($1, $2, $3, 0)
                            RETURNING id, name, min_stock, max_stock, stock
                        "#,
                    )
                    .await?,
                &[&item.name, &item.min_stock, &item.max_stock],
            )
            .await
            .map_err(|e| {
                DatabaseError(
                    Kind::Query,
                    "Error inserting item".to_owned(),
                    Some(Box::new(e)),
                )
            })?;

        if inserted_row_ids.len() != 1 {
            return Err(DatabaseError(
                Kind::UpdateError,
                "Not exactly 1 row inserted".to_owned(),
                None,
            ));
        }

        let row = inserted_row_ids.first().unwrap();

        Ok(Item {
            id: row.get("id"),
            name: row.get("name"),
            min_stock: row.get("min_stock"),
            max_stock: row.get("max_stock"),
            stock: row.get("stock"),
        })
    }
}
