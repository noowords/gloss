use std::sync::{ Arc };

use infrastructure::persistence::mysql::common::{ MySqlUnitOfWorkFactory };
use application::common::persistence::{ PoolContext, UnitOfWorkFactory };

pub fn initialize_unit_of_work_factory(
    database_type: &str,
    ctx: Arc<dyn PoolContext>
) -> Result<Arc<dyn UnitOfWorkFactory>, anyhow::Error> {
    match database_type {
        "mysql" => Ok(Arc::new(MySqlUnitOfWorkFactory::new(ctx))),
        _ => anyhow::bail!("Unsupported database type: {}", database_type)
    }
}
