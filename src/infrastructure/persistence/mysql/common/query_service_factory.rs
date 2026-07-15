
use std::sync::{ Arc };

use crate::application::{
    common::{ QueryServiceFactory },
    queries::{
        users::{ UsersQueryService }
    }
};

use super::super::queries::{ MySqlUsersQueryService };

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
