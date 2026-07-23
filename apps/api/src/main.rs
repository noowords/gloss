mod bootstrap;

use crate::bootstrap::{ Database, ApplicationBuilder };

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let application = ApplicationBuilder::new()
        .with_database(Database::MySql("mysql://root:root@localhost:3306/gloss?command_timeout=3s".to_string()))
        .build()
        .await?;

    application.serve("http", "localhost:3000").await
}
