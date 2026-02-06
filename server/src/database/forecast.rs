use chrono::NaiveDate;

use super::{
    error::{DatabaseError, Kind},
    Database,
};

#[derive(Debug, Clone)]
pub struct DailyUsage {
    pub date: NaiveDate,
    pub usage: i32,
}

impl Database {
    pub async fn get_item_usage_series(
        &self,
        item_id: i32,
        start_date: NaiveDate,
        end_date: NaiveDate,
    ) -> Result<Vec<DailyUsage>, DatabaseError> {
        let connection = self.pool.get().await?;

        let rows = connection
            .query(
                &connection
                    .prepare_cached(
                        r#"
                            SELECT day::date AS date,
                                   COALESCE(loss.quantity, 0) AS usage
                            FROM generate_series($1::date, $2::date, interval '1 day') AS day
                            LEFT JOIN stock.loss AS loss
                              ON loss.item_id = $3
                             AND loss.date = day::date
                            ORDER BY day
                        "#,
                    )
                    .await?,
                &[&start_date, &end_date, &item_id],
            )
            .await
            .map_err(|e| {
                DatabaseError(
                    Kind::Query,
                    "Error getting usage series".to_owned(),
                    Some(Box::new(e)),
                )
            })?;

        Ok(rows
            .iter()
            .map(|row| DailyUsage {
                date: row.get("date"),
                usage: row.get("usage"),
            })
            .collect())
    }
}
