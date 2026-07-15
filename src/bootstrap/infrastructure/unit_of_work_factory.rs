use std::sync::{ Arc };

use crate::domain::common::{ PoolContext, UnitOfWorkFactory };
use crate::infrastructure::persistence::mysql::common::{ MySqlUnitOfWorkFactory };

pub fn initialize_unit_of_work_factory(
    database_type: &str,
    ctx: Arc<dyn PoolContext>
) -> Result<Arc<dyn UnitOfWorkFactory>, anyhow::Error> {
    match database_type {
        "mysql" => Ok(Arc::new(MySqlUnitOfWorkFactory::new(ctx))),
        _ => anyhow::bail!("Unsupported database type: {}", database_type)
    }
}
