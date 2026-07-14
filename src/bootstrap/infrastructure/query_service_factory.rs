use std::sync::{ Arc };

use crate::application::shared::{ QueryServiceFactory };
use crate::infrastructure::mysql::shared::{ MySqlQueryServiceFactory };

pub fn initialize_query_service_factory(
    database_type: &str
) -> Result<Arc<dyn QueryServiceFactory>, anyhow::Error> {
    match database_type {
        "mysql" => Ok(Arc::new(MySqlQueryServiceFactory::new())),
        _ => Err(anyhow::anyhow!("Unsupported database type: {}", database_type))
    }
}
