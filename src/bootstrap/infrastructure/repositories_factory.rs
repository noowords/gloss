use std::sync::{ Arc };

use infrastructure::persistence::mysql::common::{ MySqlRepositoryFactory };
use application::common::persistence::{ RepositoryFactory };

pub fn initialize_repository_factory(
    database_type: &str
) -> Result<Arc<dyn RepositoryFactory>, anyhow::Error> {
    match database_type {
        "mysql" => Ok(Arc::new(MySqlRepositoryFactory::new())),
        _ => anyhow::bail!("Unsupported database type: {}", database_type)
    }
}
