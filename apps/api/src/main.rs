#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let app = bootstrap::build_http().await?;

    app.run().await
}
