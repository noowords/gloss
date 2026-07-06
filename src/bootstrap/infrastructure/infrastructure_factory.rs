use std::sync::{ Arc };

use crate::domain::shared::{ InfrastructureFactory };
use crate::infrastructure::mysql::shared::{ MySqlInfrastructureFactory };

pub fn init_infrastructure_factory(
    database_type: &str
) -> Result<Arc<dyn InfrastructureFactory>, anyhow::Error> {
    match database_type {
        "mysql" => Ok(Arc::new(MySqlInfrastructureFactory::new())),
        _ => Err(anyhow::anyhow!("Unsupported database type: {}", database_type))
    }
}
