use sqlx::{ MySqlPool };
use async_trait::{ async_trait };

use application::contracts::cqrs::command::{ CommandContextProvider, CommandContext };

use super::{ MySqlCommandContext };

pub struct MySqlCommandContextProvider {
    pool: MySqlPool
}

impl MySqlCommandContextProvider {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CommandContextProvider for MySqlCommandContextProvider {
    async fn provide_context(&self) -> Result<Box<dyn CommandContext>, anyhow::Error> {
        let tx = self.pool.begin().await?;

        Ok(Box::new(MySqlCommandContext::new(tx)))
    }
}
