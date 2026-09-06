use tokio::net::{ TcpListener };
use axum::{ Router };

pub async fn serve_http(listener: TcpListener, router: Router) -> Result<(), anyhow::Error> {
    axum::serve(listener, router)
        .await
        .map_err(|e| anyhow::anyhow!("Failed to run axum HTTP server: {}: {e}", e.to_string()))
}
