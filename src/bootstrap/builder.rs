use std::sync::{ Arc };

use crate::providers::{ AppState };

use super::infrastructure::{
    connect_to_database,
    initialize_unit_of_work_factory,
    initialize_command_repository_factory,
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

    pub fn with_database(mut self, db_type: impl Into<String>, db_url: impl Into<String>) -> Self {
        self.database = Some((db_type.into(), db_url.into()));
        self
    }

    pub async fn build(self) -> Result<Application, anyhow::Error> {
        let (database_type, database_url) = self.database
            .ok_or_else(|| anyhow::anyhow!("Database is not configured. Call .with_database()"))?;

        let ctx = connect_to_database(&database_type, &database_url).await?;
        let uow_factory = initialize_unit_of_work_factory(&database_type, ctx.clone())?;
        let cr_factory = initialize_command_repository_factory(&database_type)?;
        let qs_factory = initialize_query_service_factory(&database_type)?;

        let command_bus = build_command_bus(uow_factory.clone(), cr_factory.clone());
        let query_bus = build_query_bus(ctx.clone(), qs_factory.clone());
    
        Ok(Application::new(Arc::new(AppState::new(command_bus, query_bus))))
    }
}
