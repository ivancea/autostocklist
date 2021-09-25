pub mod error;
pub mod item;
pub mod movements;

use error::{Error, Kind};
use log::debug;
use deadpool_postgres::{Config, ManagerConfig, Pool, RecyclingMethod};
use postgres::NoTls;

#[derive(Clone)]
pub struct Database {
    pool: Pool,
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

        let mut config = Config::new();
        
        config.dbname = Some(database.as_ref().to_owned());
        config.host = Some(host.as_ref().to_owned());
        config.port = Some(port);
        config.user = Some(user.as_ref().to_owned());
        config.password = Some(password.as_ref().to_owned());
        config.manager = Some(ManagerConfig { recycling_method: RecyclingMethod::Fast });
        

        let pool = config.create_pool(NoTls)?;

        pool.get().await?
            .simple_query("SELECT 1").await
            .map_err(|e| Error(
                Kind::Connection,
                "Error checking connection".to_owned(),
                Some(Box::new(e))
            ))?;

        Ok(Database { pool })
    }
}