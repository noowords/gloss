use std::sync::{ Arc };

use crate::domain::shared::{ CommandRepositoryFactory };
use crate::infrastructure::mysql::shared::{ MySqlCommandRepositoryFactory };

pub fn initialize_command_repository_factory(
    database_type: &str
) -> Result<Arc<dyn CommandRepositoryFactory>, anyhow::Error> {
    match database_type {
        "mysql" => Ok(Arc::new(MySqlCommandRepositoryFactory::new())),
        _ => Err(anyhow::anyhow!("Unsupported database type: {}", database_type))
    }
}
