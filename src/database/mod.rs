pub mod error;

use actix_web::web;
use chrono::NaiveDate;
use error::{Error, Kind};
use log::debug;
use r2d2::Pool;
use r2d2_postgres::{
    postgres::{Config, NoTls},
    PostgresConnectionManager,
};
use std::time::{Duration, Instant};

macro_rules! measure {
    ($message:expr, $expression:expr) => {{
        let instant = Instant::now();

        let value = $expression;

        debug!("{}. Elapsed time: {:#?}", $message, instant.elapsed());

        value
    }};
}

#[derive(Clone)]
pub struct Database {
    pool: Pool<PostgresConnectionManager<NoTls>>,
}

impl Database {
    pub async fn new(
        host: impl AsRef<str>,
        port: u16,
        database: impl AsRef<str>,
        user: impl AsRef<str>,
        password: impl AsRef<str>,
    ) -> Result<Database, Error> {
        debug!("Connecting with:");
        debug!("    - Host: {}", host.as_ref());
        debug!("    - Port: {}", port);
        debug!("    - Database: {}", database.as_ref());
        debug!("    - User: {}", user.as_ref());

        let connection_manager = PostgresConnectionManager::new(
            Config::new()
                .host(host.as_ref())
                .port(port)
                .user(user.as_ref())
                .password(password.as_ref())
                .dbname(database.as_ref())
                .to_owned(),
            NoTls,
        );

        let pool = measure!(
            "Connection created",
            Pool::builder()
                .build(connection_manager)
                .map_err(|e| Error(
                    Kind::Connection,
                    "Error connecting to database".to_owned(),
                    Some(Box::new(e))
                ))?
        );

        let temp_pool = pool.clone();

        web::block(move || {
            measure!(
                "Connection check passed",
                temp_pool
                    .get()?
                    .is_valid(Duration::from_secs(5))
                    .map_err(|e| Error(
                        Kind::Connection,
                        "Error checking connection".to_owned(),
                        Some(Box::new(e))
                    ))
            )
        })
        .await
        .unwrap()?;

        Ok(Database { pool })
    }

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

        let message = format!(
            "Executed query with: item_id:{}, date:{}, quantity:{}",
            item_id, date, quantity
        );

        let pool = self.pool.clone();

        web::block(move || -> Result<(), Error> {
            let modified_rows = measure!(
                message,
                pool.get()?
                    .execute(
                        r#"
                        INSERT INTO stock_movements (item_id, date, quantity)
                        VALUES($1, $2, $3)
                        ON CONFLICT (item_id, date)
                        DO
                        UPDATE SET quantity = stock_movements.quantity + excluded.quantity
                    "#,
                        &[&item_id, &date, &quantity]
                    )
                    .map_err(|e| Error(
                        Kind::Query,
                        "Error inserting movement".to_owned(),
                        Some(Box::new(e))
                    ))?
            );

            if modified_rows > 0 {
                Ok(())
            } else {
                Err(Error(
                    Kind::Query,
                    "No modified rows in INSERT or UPDATE statement".to_owned(),
                    None,
                ))
            }
        })
        .await
        .unwrap()
    }
}
