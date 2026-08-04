use sqlx::{ MySqlPool };

use application::contracts::cqrs::query::{ QueryContextProvider, QueryContext };

use super::{ MySqlQueryContext };

pub struct MySqlQueryContextProvider {
    pool: MySqlPool
}

impl MySqlQueryContextProvider {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

impl QueryContextProvider for MySqlQueryContextProvider {
    fn provide_context(&self) -> Box<dyn QueryContext> {
        let pool = self.pool.clone();

        Box::new(MySqlQueryContext::new(pool))
    }
}
