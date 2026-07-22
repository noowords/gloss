use sqlx::{ MySqlPool };

use application::{ QueryProvider, QueryContext };

use crate::persistence::mysql::{ MySqlQueryContext };

pub struct MySqlQueryProvider {
    pool: MySqlPool
}

impl MySqlQueryProvider {
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }
}

impl QueryProvider for MySqlQueryProvider {
    fn provide_context(&self) -> Box<dyn QueryContext> {
        let pool = self.pool.clone();
        
        Box::new(MySqlQueryContext::new(pool))
    }
}
