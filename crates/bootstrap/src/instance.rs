use std::sync::{ Arc };
use tokio::net::{ TcpListener };

use application::pipeline::{
    command::{ CommandBus },
    query::{ QueryBus }
};
use presentation::http::{ HttpState, create_router, serve };

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
            "http" => {
                let state = HttpState::new(self.command_bus.clone(), self.query_bus.clone());
                let router = create_router(state);
            
                let listener = TcpListener::bind(addr)
                    .await
                    .map_err(|e| anyhow::anyhow!("TcpListener binding failed: {}", e.to_string()))?;
            
                serve(listener, router).await
            },
            _ => anyhow::bail!("Unsupported server type: {}", server_type),
        }
    }
}
