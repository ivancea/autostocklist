use super::error::{DatabaseError, Kind};
use super::Database;
use chrono::NaiveDate;

impl Database {
    pub async fn update_stock_loss(
        &self,
        item_id: i32,
        date: NaiveDate,
        quantity: i32,
    ) -> Result<(), DatabaseError> {
        let connection = self.pool.get().await?;

        connection
            .execute(
                &connection
                    .prepare_cached(
                        r#"
                            CALL stock.update_stock_loss($1, $2, $3)
                        "#,
                    )
                    .await?,
                &[&item_id, &date, &quantity],
            )
            .await
            .map_err(|e| {
                DatabaseError(
                    Kind::Query,
                    "Error changing stock loss".to_owned(),
                    Some(Box::new(e)),
                )
            })?;

        Ok(())
    }

    pub async fn update_stock_resupply(
        &self,
        item_id: i32,
        date: NaiveDate,
        quantity: i32,
    ) -> Result<(), DatabaseError> {
        let connection = self.pool.get().await?;

        connection
            .execute(
                &connection
                    .prepare_cached(
                        r#"
                            CALL stock.update_stock_resupply($1, $2, $3)
                        "#,
                    )
                    .await?,
                &[&item_id, &date, &quantity],
            )
            .await
            .map_err(|e| {
                DatabaseError(
                    Kind::Query,
                    "Error changing stock resupply".to_owned(),
                    Some(Box::new(e)),
                )
            })?;

        Ok(())
    }
}
