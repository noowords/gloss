use std::sync::{ Arc };
use tokio::net::{ TcpListener };

use crate::presentation::http::{ HttpState, create_router, serve };

use crate::providers::{ AppState };

pub async fn serve_http(
    addr: &str,
    app_state: Arc<AppState>
) -> Result<(), anyhow::Error> {
    let state = HttpState::new(
        app_state.command_bus.clone(),
        app_state.query_bus.clone(),
    );
    let router = create_router(state);
    
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|e| anyhow::anyhow!("TcpListener binding failed: {}", e.to_string()))?;

    serve(listener, router).await
}
