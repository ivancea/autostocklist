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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Duration;
    use postgres::{Client, NoTls};
    use std::thread;
    use std::process::Command;
    use std::time::Duration as StdDuration;
    use testcontainers::{clients::Cli, images::postgres::Postgres};

    fn connect_with_retry(conn_str: &str) -> Client {
        let mut last_error = None;

        for _ in 0..10 {
            match Client::connect(conn_str, NoTls) {
                Ok(client) => return client,
                Err(error) => {
                    last_error = Some(error);
                    thread::sleep(StdDuration::from_millis(500));
                }
            }
        }

        panic!("Failed to connect to Postgres: {:?}", last_error);
    }

    fn docker_available() -> bool {
        Command::new("docker")
            .arg("version")
            .output()
            .map(|output| output.status.success())
            .unwrap_or(false)
    }

    #[actix_web::test]
    async fn get_item_usage_series_fills_missing_days() {
        if !docker_available() {
            eprintln!("Docker not available, skipping integration test.");
            return;
        }

        let docker = Cli::default();
        let node = docker.run(Postgres::default());
        let port = node.get_host_port(5432);
        let conn_str = format!(
            "postgresql://postgres:postgres@127.0.0.1:{}/postgres",
            port
        );

        let mut client = connect_with_retry(&conn_str);
        let schema_sql = include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../database/scripts/01_create_stock_database.sql"
        ));
        client
            .batch_execute(schema_sql)
            .expect("create schema");

        let item_row = client
            .query_one(
                "INSERT INTO stock.item (name, min_stock, max_stock, stock) VALUES ($1, $2, $3, $4) RETURNING id",
                &[&"Test item", &1, &10, &5],
            )
            .expect("insert item");
        let item_id: i32 = item_row.get("id");

        let start_date = NaiveDate::from_ymd_opt(2023, 1, 2).expect("valid date");
        let day_one = start_date + Duration::days(1);
        let day_four = start_date + Duration::days(4);
        client
            .execute(
                "INSERT INTO stock.loss (item_id, date, quantity) VALUES ($1, $2, $3)",
                &[&item_id, &day_one, &2],
            )
            .expect("insert loss");
        client
            .execute(
                "INSERT INTO stock.loss (item_id, date, quantity) VALUES ($1, $2, $3)",
                &[&item_id, &day_four, &4],
            )
            .expect("insert loss");

        let database = Database::new("127.0.0.1", port, "postgres", "postgres", "postgres")
            .await
            .expect("connect database");
        let end_date = start_date + Duration::days(6);
        let usage = database
            .get_item_usage_series(item_id, start_date, end_date)
            .await
            .expect("load usage");

        assert_eq!(usage.len(), 7);
        assert_eq!(usage[0].date, start_date);
        assert_eq!(usage[0].usage, 0);
        assert_eq!(usage[1].usage, 2);
        assert_eq!(usage[4].usage, 4);
        assert_eq!(usage[6].usage, 0);
    }
}
