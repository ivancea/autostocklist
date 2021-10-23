use std::fmt::Display;

use deadpool_postgres::{CreatePoolError, PoolError};

#[derive(Debug, Clone, Copy)]
pub enum Kind {
    Connection,
    Query,
    ItemNotFound,
}

#[derive(Debug)]
pub struct DatabaseError(
    pub Kind,
    pub String,
    pub Option<Box<dyn std::error::Error + Send>>,
);

impl std::error::Error for DatabaseError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.2.as_ref().map(|e| &**e as _)
    }
}

impl Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.0 {
            Kind::Connection => write!(f, "Connection error")?,
            Kind::Query => write!(f, "Query execution error")?,
            Kind::ItemNotFound => write!(f, "Item not found")?,
        };

        if let Some(ref cause) = self.2 {
            write!(f, ": {}", cause)?;
        }

        Ok(())
    }
}

impl From<CreatePoolError> for DatabaseError {
    fn from(error: CreatePoolError) -> Self {
        DatabaseError(
            Kind::Connection,
            "Error creating pool".to_owned(),
            Some(Box::new(error)),
        )
    }
}

impl From<PoolError> for DatabaseError {
    fn from(error: PoolError) -> Self {
        DatabaseError(
            Kind::Connection,
            "Pool error".to_owned(),
            Some(Box::new(error)),
        )
    }
}

impl From<postgres::Error> for DatabaseError {
    fn from(error: postgres::Error) -> Self {
        DatabaseError(Kind::Connection, "".to_owned(), Some(Box::new(error)))
    }
}
