use std::sync::{ Arc };

use super::presentation::{ serve_http };
use crate::application::shared::{ CommandBus, QueryBus };

pub struct Application {
    command_bus: Arc<CommandBus>,
    query_bus: Arc<QueryBus>
}

impl Application {
    pub fn new(command_bus: Arc<CommandBus>, query_bus: Arc<QueryBus>) -> Self {
        Self { command_bus, query_bus }
    }
    
    pub async fn serve(&self, server_type: &str, addr: &str) -> Result<(), anyhow::Error> {
        match server_type {
            "http" => serve_http(addr, self.command_bus.clone(), self.query_bus.clone()).await,
            _ => anyhow::bail!("Unsupported server type: {}", server_type),
        }
    }
}
