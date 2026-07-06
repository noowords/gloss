use std::sync::{ Arc };
use tokio::net::{ TcpListener };

use crate::application::shared::{ CommandBus };
use crate::presentation::http::{ create_state, create_router, serve };

pub async fn serve_http(addr: &str, command_bus: Arc<CommandBus>) -> Result<(), anyhow::Error> {
    let state = create_state(command_bus);
    let router = create_router(state);
    
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|e| anyhow::anyhow!("TcpListener binding failed: {}", e.to_string()))?;

    serve(listener, router).await
}
