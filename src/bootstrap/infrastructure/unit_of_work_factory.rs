use std::sync::{ Arc };

use crate::domain::shared::{ PoolContext, UnitOfWorkFactory };
use crate::infrastructure::mysql::shared::{ MySqlUnitOfWorkFactory };

pub fn init_unit_of_work_factory(
    database_type: &str,
    database_pool: Arc<dyn PoolContext>
) -> Result<Arc<dyn UnitOfWorkFactory>, anyhow::Error> {
    match database_type {
        "mysql" => Ok(Arc::new(MySqlUnitOfWorkFactory::new(database_pool))),
        _ => Err(anyhow::anyhow!("Unsupported database type: {}", database_type))
    }
}
