use std::fmt::Display;

use deadpool_postgres::{PoolError, config::ConfigError};

#[derive(Debug, Clone, Copy)]
pub enum Kind {
    Connection,
    Query,
    ItemNotFound,
}

#[derive(Debug)]
pub struct Error(
    pub Kind,
    pub String,
    pub Option<Box<dyn std::error::Error + Send>>,
);

impl std::error::Error for Error {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        self.2.as_ref().map(|e| &**e as _)
    }
}

impl Display for Error {
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

impl From<ConfigError> for Error {
    fn from(error: ConfigError) -> Self {
        Error(Kind::Connection, "Configuration error".to_owned(), Some(Box::new(error)))
    }
}

impl From<PoolError> for Error {
    fn from(error: PoolError) -> Self {
        Error(Kind::Connection, "Pool error".to_owned(), Some(Box::new(error)))
    }
}

impl From<postgres::Error> for Error {
    fn from(error: postgres::Error) -> Self {
        Error(Kind::Connection, "".to_owned(), Some(Box::new(error)))
    }
}
