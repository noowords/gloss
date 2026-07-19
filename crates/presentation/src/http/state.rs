use std::sync::{ Arc };

use application::buses::{
    command_bus::{ CommandBus },
    query_bus::{ QueryBus }
};

#[derive(Clone)]
pub struct HttpState {
    pub command_bus: Arc<CommandBus>,
    pub query_bus: Arc<QueryBus>
}

impl HttpState {
    pub fn new(
        command_bus: Arc<CommandBus>,
        query_bus: Arc<QueryBus>
    ) -> Self {
        Self { command_bus, query_bus }
    }
}
