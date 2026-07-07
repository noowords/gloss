use std::sync::{ Arc };

use crate::application::shared::{ CommandBus, QueryBus };

#[derive(Clone)]
pub struct HttpState {
    pub command_bus: Arc<CommandBus>,
    pub query_bus: Arc<QueryBus>
}

pub fn create_state(
    command_bus: Arc<CommandBus>,
    query_bus: Arc<QueryBus>
) -> HttpState {
    HttpState { command_bus, query_bus }
}
