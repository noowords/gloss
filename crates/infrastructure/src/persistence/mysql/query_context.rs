use sqlx::{ MySqlPool };

use application::{ QueryContext };

pub struct MySqlQueryContext {
    pool: MySqlPool
}

impl MySqlQueryContext {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

impl QueryContext for MySqlQueryContext { }
