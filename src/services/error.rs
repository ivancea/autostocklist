use crate::database::error::DatabaseError;
use std::fmt::Display;

#[derive(Debug)]
pub enum ServiceError {
    Input(String),
    Database(DatabaseError),
}

impl std::error::Error for ServiceError {
    fn cause(&self) -> Option<&dyn std::error::Error> {
        match &self {
            ServiceError::Database(error) => Some(error),
            _ => None,
        }
    }
}

impl Display for ServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            ServiceError::Input(msg) => write!(f, "Invalid input: {}", msg)?,
            ServiceError::Database(error) => write!(f, "Database error: {}", error)?,
        };

        Ok(())
    }
}

impl From<DatabaseError> for ServiceError {
    fn from(error: DatabaseError) -> Self {
        ServiceError::Database(error)
    }
}
