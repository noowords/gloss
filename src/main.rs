mod domain;
mod infrastructure;
mod application;
mod presentation;
mod bootstrap;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let pool = bootstrap::infrastructure::connect_to_database("mysql", "mysql://root:root@localhost:3306/gloss").await?;
    let uow_factory = bootstrap::infrastructure::init_unit_of_work_factory("mysql", pool)?;
    let infra_factory = bootstrap::infrastructure::init_infrastructure_factory("mysql")?;
    
    let command_bus = bootstrap::application::build_command_bus(uow_factory, infra_factory);

    bootstrap::presentation::serve_http("0.0.0.0:3000", command_bus).await
}
