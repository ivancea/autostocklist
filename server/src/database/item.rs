use crate::dtos::item_dtos::{Item, NewItemRequest, UpdateItemRequest};

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

        let row_option = connection
            .query_opt(
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

        if row_option.is_none() {
            return Err(DatabaseError(Kind::ItemNotFound, "".to_owned(), None));
        }

        let row = row_option.unwrap();

        Ok(Item {
            id: row.get("id"),
            name: row.get("name"),
            min_stock: row.get("min_stock"),
            max_stock: row.get("max_stock"),
            stock: row.get("stock"),
        })
    }

    pub async fn update_item(&self, item: UpdateItemRequest) -> Result<Item, DatabaseError> {
        let connection = self.pool.get().await?;

        let row_option = connection
            .query_opt(
                &connection
                    .prepare_cached(
                        r#"
                            UPDATE stock.item
                            SET name = $1, min_stock = $2, max_stock = $3
                            WHERE id = $4
                            RETURNING id, name, min_stock, max_stock, stock
                        "#,
                    )
                    .await?,
                &[&item.name, &item.min_stock, &item.max_stock, &item.id],
            )
            .await
            .map_err(|e| {
                DatabaseError(
                    Kind::Query,
                    "Error updating item".to_owned(),
                    Some(Box::new(e)),
                )
            })?;

        if row_option.is_none() {
            return Err(DatabaseError(Kind::ItemNotFound, "".to_owned(), None));
        }

        let row = row_option.unwrap();

        Ok(Item {
            id: row.get("id"),
            name: row.get("name"),
            min_stock: row.get("min_stock"),
            max_stock: row.get("max_stock"),
            stock: row.get("stock"),
        })
    }

    pub async fn remove_item(&self, item_id: i32) -> Result<(), DatabaseError> {
        let connection = self.pool.get().await?;

        let removed_rows = connection
            .execute(
                &connection
                    .prepare_cached(
                        r#"
                            DELETE FROM stock.item
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
                    "Error removing item".to_owned(),
                    Some(Box::new(e)),
                )
            })?;

        if removed_rows == 0 {
            return Err(DatabaseError(Kind::ItemNotFound, "".to_owned(), None));
        }

        Ok(())
    }
}
