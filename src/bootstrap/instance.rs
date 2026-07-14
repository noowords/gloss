use std::sync::{ Arc };

use crate::providers::{ AppState };

use super::presentation::{ serve_http };

pub struct Application {
    state: Arc<AppState>
}

impl Application {
    pub fn new(state: Arc<AppState>) -> Self {
        Self { state }
    }
    
    pub async fn serve(&self, server_type: &str, addr: &str) -> Result<(), anyhow::Error> {
        match server_type {
            "http" => serve_http(addr, self.state.clone()).await,
            _ => anyhow::bail!("Unsupported server type: {}", server_type),
        }
    }
}
