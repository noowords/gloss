
use std::sync::{ Arc };

use application::{
    persistence::factories::{ QueryServiceFactory },
    queries::{
        users::{ UsersQueryService }
    }
};

use super::super::query_services::{ MySqlUsersQueryService };

#[derive(Default)]
pub struct MySqlQueryServiceFactory;

impl MySqlQueryServiceFactory {
    pub fn new() -> Self {
        Self::default()
    }
}

impl QueryServiceFactory for MySqlQueryServiceFactory {
    fn users_service(&self) -> Arc<dyn UsersQueryService> {
        Arc::new(MySqlUsersQueryService::new())
    }
}
