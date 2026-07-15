use std::sync::{ Arc };

use crate::application::common::{ QueryServiceFactory };
use crate::infrastructure::persistence::mysql::common::{ MySqlQueryServiceFactory };

pub fn initialize_query_service_factory(
    database_type: &str
) -> Result<Arc<dyn QueryServiceFactory>, anyhow::Error> {
    match database_type {
        "mysql" => Ok(Arc::new(MySqlQueryServiceFactory::new())),
        _ => anyhow::bail!("Unsupported database type: {}", database_type)
    }
}
