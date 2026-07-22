use sqlx::{ MySqlPool };
use async_trait::{ async_trait };

use application::{ CommandProvider, CommandContext };

use crate::persistence::mysql::{ MySqlCommandContext };

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
