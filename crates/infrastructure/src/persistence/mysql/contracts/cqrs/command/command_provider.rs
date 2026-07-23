use sqlx::{ MySqlPool };
use async_trait::{ async_trait };

use application::contracts::cqrs::command::{ CommandProvider, CommandContext };

use super::{ MySqlCommandContext };

pub struct MySqlCommandProvider {
    pool: MySqlPool
}

impl MySqlCommandProvider {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl CommandProvider for MySqlCommandProvider {
    async fn provide_context(&self) -> Result<Box<dyn CommandContext>, anyhow::Error> {
        let tx = self.pool.begin().await?;

        Ok(Box::new(MySqlCommandContext::new(tx)))
    }
}
