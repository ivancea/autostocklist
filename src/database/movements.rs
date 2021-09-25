use chrono::NaiveDate;
use log::debug;

use super::Database;
use super::error::{Error, Kind};

impl Database {
    pub async fn reduce_stock(
        &self,
        item_id: i32,
        date: NaiveDate,
        quantity: i32,
    ) -> Result<(), Error> {
        if quantity <= 0 {
            return Err(Error(
                Kind::Query,
                "Quantity must be a non-zero positive number".to_owned(),
                None,
            ));
        }

        debug!("Reducing stock: item_id:{}, date:{}, quantity:{}", item_id, date, quantity);

        let connection = self.pool.get().await?;

        let modified_rows = connection
            .execute(
                &connection.prepare_cached(
                    r#"
                        INSERT INTO stock_movement (item_id, date, quantity)
                        VALUES($1, $2, $3)
                        ON CONFLICT (item_id, date)
                        DO
                        UPDATE SET quantity = stock_movement.quantity + excluded.quantity
                    "#
                ).await?,
                &[&item_id, &date, &quantity]
            ).await
            .map_err(|e| Error(
                Kind::Query,
                "Error inserting movement".to_owned(),
                Some(Box::new(e))
            ))?;

        if modified_rows == 1 {
            Ok(())
        } else {
            Err(Error(
                Kind::Query,
                "Not exactly 1 modified rows in INSERT or UPDATE statement".to_owned(),
                None,
            ))
        }
    }
}