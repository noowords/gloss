use std::sync::Arc;
use tokio::net::TcpListener;

use application::common::{
    QueryBus,
    commands::{ CommandBus }
};
use presentation::http::{HttpState, create_router, serve};

pub async fn serve_http(
    addr: &str,
    command_bus: Arc<CommandBus>,
    query_bus: Arc<QueryBus>,
) -> Result<(), anyhow::Error> {
    let state = HttpState::new(command_bus.clone(), query_bus.clone());
    let router = create_router(state);

    let listener = TcpListener::bind(addr)
        .await
        .map_err(|e| anyhow::anyhow!("TcpListener binding failed: {}", e.to_string()))?;

    serve(listener, router).await
}
