mod bootstrap;

use crate::bootstrap::{ ApplicationBuilder };

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let application = ApplicationBuilder::new()
        .with_database("mysql", "mysql://root:root@localhost:3306/gloss?command_timeout=3s")
        .build()
        .await?;

    application.serve("http", "localhost:3000").await
}
