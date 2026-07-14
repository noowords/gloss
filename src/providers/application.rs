use std::sync::{ Arc };

use crate::application::shared::{ CommandBus, QueryBus };

#[derive(Clone)]
pub struct AppState {
    pub command_bus: Arc<CommandBus>,
    pub query_bus: Arc<QueryBus>
}

impl AppState {
    pub fn new(
        command_bus: Arc<CommandBus>,
        query_bus: Arc<QueryBus>
    ) -> Self {
        Self { command_bus, query_bus }
    }
}
