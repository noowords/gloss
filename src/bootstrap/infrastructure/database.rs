use std::sync::{ Arc };
use sqlx::mysql::{ MySqlPool };

use crate::domain::shared::{ PoolContext };
use crate::infrastructure::mysql::shared::{ MySqlPoolContext };

pub async fn connect_to_database(
    database_type: &str,
    database_url: &str
) -> Result<Arc<dyn PoolContext>, anyhow::Error> {
    match database_type {
        "mysql" => {
            let pool = MySqlPool::connect(database_url)
                .await
                .map_err(|e| anyhow::anyhow!("Database connection failed: {}", e.to_string()))?;

            Ok(Arc::new(MySqlPoolContext::new(pool)))
        },
        _ => Err(anyhow::anyhow!("Unsupported database type: {}", database_type))
    }
}
