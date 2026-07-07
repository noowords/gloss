mod domain;
mod infrastructure;
mod application;
mod presentation;
mod bootstrap;

use crate::bootstrap::{
    infrastructure::{ connect_to_database, init_unit_of_work_factory, init_infrastructure_factory },
    application::{ build_command_bus, build_query_bus },
    presentation::{ serve_http }
};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let pool = connect_to_database("mysql", "mysql://root:root@localhost:3306/gloss").await?;
    let uow_factory = init_unit_of_work_factory("mysql", pool)?;
    let infra_factory = init_infrastructure_factory("mysql")?;
    
    let command_bus = build_command_bus(uow_factory.clone(), infra_factory.clone());
    let query_bus = build_query_bus(uow_factory, infra_factory);

    serve_http("0.0.0.0:3000", command_bus, query_bus).await
}
