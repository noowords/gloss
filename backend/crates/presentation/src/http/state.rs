use std::sync::{ Arc };

use application::{
    contracts::{ TokenService },
    pipeline::{
        command::{ CommandBus },
        query::{ QueryBus }
    }
};

#[derive(Clone)]
pub struct HttpState {
    pub command_bus: Arc<CommandBus>,
    pub query_bus: Arc<QueryBus>,
    pub token_service: Arc<dyn TokenService>
}

impl HttpState {
    pub fn new(
        command_bus: Arc<CommandBus>,
        query_bus: Arc<QueryBus>,
        token_service: Arc<dyn TokenService>
    ) -> Self {
        Self { command_bus, query_bus, token_service }
    }
}
