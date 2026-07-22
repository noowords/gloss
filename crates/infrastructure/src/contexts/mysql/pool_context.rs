use std::any::{ Any };
use sqlx::mysql::{ MySqlPool };

use application::contexts::{ PoolContext };

pub struct MySqlPoolContext {
    pub pool: MySqlPool
}

impl MySqlPoolContext {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

impl PoolContext for MySqlPoolContext {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
