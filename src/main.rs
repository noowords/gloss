mod domain;
mod infrastructure;
mod application;
mod presentation;
mod providers;
mod bootstrap;

use crate::bootstrap::{
    InfrastructureBuilder, build_application,
    presentation::{ serve_http }
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let infrastructure = InfrastructureBuilder::new()
        .with_database("mysql", "mysql://root:root@localhost:3306/gloss")
        .build()
        .await?;

    let application = build_application(infrastructure)?;

    serve_http("127.0.0.1:3000", application).await
}
