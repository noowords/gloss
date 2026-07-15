use std::sync::{ Arc };

use crate::infrastructure::persistence::mysql::common::{ MySqlRepositoryFactory };
use crate::application::common::persistence::{ RepositoryFactory };

pub fn initialize_repository_factory(
    database_type: &str
) -> Result<Arc<dyn RepositoryFactory>, anyhow::Error> {
    match database_type {
        "mysql" => Ok(Arc::new(MySqlRepositoryFactory::new())),
        _ => anyhow::bail!("Unsupported database type: {}", database_type)
    }
}
