use std::any::{ Any };
use sqlx::{ MySqlPool };

use application::interfaces::query::{ QueryContext };

pub struct MySqlQueryContext {
    pub pool: MySqlPool
}

impl MySqlQueryContext {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &MySqlPool {
        &self.pool
    }
}

impl QueryContext for MySqlQueryContext {
    fn as_any(&self) -> &dyn Any { self }
}
