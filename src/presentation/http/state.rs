use std::sync::{ Arc };

use crate::application::shared::{ CommandBus };

#[derive(Clone)]
pub struct HttpState {
    pub command_bus: Arc<CommandBus>
}

pub fn create_state(command_bus: Arc<CommandBus>) -> HttpState {
    HttpState { command_bus }
}
