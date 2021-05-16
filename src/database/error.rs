use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Kind {
    Connection,
    Query,
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
        };

        if let Some(ref cause) = self.2 {
            write!(f, ": {}", cause)?;
        }

        Ok(())
    }
}

impl From<r2d2::Error> for Error {
    fn from(error: r2d2::Error) -> Self {
        Error(Kind::Connection, "".to_owned(), Some(Box::new(error)))
    }
}
