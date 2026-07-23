use std::sync::{ Arc };
use sqlx::{ MySqlPool };

use crate::adapters::mysql::interfaces::{
    command::{ MySqlCommandProvider },
    query::{ MySqlQueryProvider }
};
use crate::{ MySqlConnection };

pub struct MySqlDatabaseProvider {
    pool: MySqlPool
}

impl MySqlDatabaseProvider {
    pub async fn connect(url: &str) -> Result<Self, anyhow::Error> {
        let pool = MySqlConnection::connect(url).await?;
        
        Ok(Self { pool })
    }

    pub fn command_provider(&self) -> Arc<MySqlCommandProvider> {
        Arc::new(MySqlCommandProvider::new(self.pool()))
    }

    pub fn query_provider(&self) -> Arc<MySqlQueryProvider> {
        Arc::new(MySqlQueryProvider::new(self.pool()))
    }
    
    pub fn pool(&self) -> MySqlPool {
        self.pool.clone()
    }
}
