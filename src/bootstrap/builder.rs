use super::infrastructure::{
    connect_to_database,
    initialize_unit_of_work_factory,
    initialize_repository_factory,
    initialize_query_service_factory
};

use super::{
    Application,
    application::{ build_command_bus, build_query_bus }
};

#[derive(Default)]
pub struct ApplicationBuilder {
    database: Option<(String, String)>
}

impl ApplicationBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_database(mut self, database_type: impl Into<String>, database_url: impl Into<String>) -> Self {
        self.database = Some((database_type.into(), database_url.into()));
        self
    }

    pub async fn build(self) -> Result<Application, anyhow::Error> {
        let (database_type, database_url) = self.database
            .ok_or_else(|| anyhow::anyhow!("Database is not configured. Call .with_database()"))?;

        let ctx = connect_to_database(&database_type, &database_url).await?;
        let uow_factory = initialize_unit_of_work_factory(&database_type, ctx.clone())?;
        let repository_factory = initialize_repository_factory(&database_type)?;
        let query_service_factory = initialize_query_service_factory(&database_type)?;

        let command_bus = build_command_bus(uow_factory.clone(), repository_factory.clone());
        let query_bus = build_query_bus(ctx.clone(), query_service_factory.clone());
    
        Ok(Application::new(command_bus, query_bus))
    }
}
