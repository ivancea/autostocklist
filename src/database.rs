use std::{fmt::Display, time::Instant};

use chrono::NaiveDate;
use itertools::Itertools;
use log::debug;
use scylla::{Session, SessionBuilder, batch::Consistency, frame::value::Counter, prepared_statement::PreparedStatement, transport::errors::{NewSessionError, QueryError}};

macro_rules! measure {
    ($message:expr, $expression:expr) => {
        {
            let instant = Instant::now();

            $expression;

            debug!("{}. Elapsed time: {:#?}", $message, instant.elapsed());
        }
    };
}

pub struct Database {
    session: Session,

    reduce_prepared: PreparedStatement,
}

impl Database {
    pub async fn new(nodes: &[impl AsRef<str> + Display]) -> Result<Database, NewSessionError> {
        assert!(!nodes.is_empty(), "No nodes supplied");

        debug!("Connecting to database nodes [{}]", nodes.iter().join(","));
        let mut builder = SessionBuilder::new();

        for node in nodes {
            builder = builder.known_node(node);
        }

        let session = builder.build().await?;

        session.query("SELECT now() FROM system.local", []).await?;

        let mut reduce_prepared = session
            .prepare(
                r#"
                UPDATE stock.stock_movements
                SET amount = amount + ?
                WHERE user = ?
                AND item = ?
                AND date = ?
            "#,
            )
            .await?;

        reduce_prepared.set_consistency(Consistency::Quorum);

        Ok(Database { session, reduce_prepared })
    }

    pub async fn reduce_stock(
        &self,
        user: u32,
        item: u32,
        date: &NaiveDate,
        amount: u32,
    ) -> Result<(), QueryError> {
        assert!(amount > 0, "Amount cannot be 0");

        let message = format!(
            "Executed query with: user:{}, item:{}, date:{}, amount:{}",
            user, item, date, amount
        );

        measure!(
            message,
            self.session
                .execute(
                    &self.reduce_prepared,
                    (Counter(amount.into()), user as i32, item as i32, date),
                )
                .await?
        );

        Ok(())
    }
}