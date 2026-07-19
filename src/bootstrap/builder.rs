use std::sync::{ Arc };
use sqlx::mysql::{ MySqlPool };

use application::{
    persistence::factories::{ QueryServiceFactory },
    buses::{
        command_bus::{ CommandBus },
        query_bus::{ QueryBus }
    },
    queries::{
        users::{
            get::{ GetUsersQuery, GetUsersHandler },
            get_by_id::{ GetUserByIdQuery, GetUserByIdHandler },
            get_profile_by_id::{ GetUserProfileByIdQuery, GetUserProfileByIdHandler }
        }
    }
};
use infrastructure::persistence::mysql::{
    contexts::{ MySqlPoolContext },
    factories::{ MySqlRepositoryFactory, MySqlQueryServiceFactory, MySqlUnitOfWorkFactory }
};

use super::{ Application };

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

        let ctx = match database_type.as_str() {
            "mysql" => {
                let pool = MySqlPool::connect(&database_url)
                    .await
                    .map_err(|e| anyhow::anyhow!("Database connection failed: {}", e.to_string()))?;
    
                Arc::new(MySqlPoolContext::new(pool))
            },
            _ => anyhow::bail!("Unsupported database type: {}", &database_type)
        };
        
        let uow_factory = match database_type.as_str() {
            "mysql" => Arc::new(MySqlUnitOfWorkFactory::new(ctx.clone())),
            _ => anyhow::bail!("Unsupported database type: {}", &database_type)
        };
        
        let repository_factory = match database_type.as_str() {
            "mysql" => Arc::new(MySqlRepositoryFactory::new()),
            _ => anyhow::bail!("Unsupported database type: {}", database_type)
        };

        let query_service_factory = match database_type.as_str() {
            "mysql" => Arc::new(MySqlQueryServiceFactory::new()),
            _ => anyhow::bail!("Unsupported database type: {}", database_type)
        };

        let command_bus = CommandBus::new(uow_factory, repository_factory);

        let mut query_bus = QueryBus::new();

        query_bus.register::<GetUsersQuery, GetUsersHandler>(
            GetUsersHandler::new(
                ctx.clone(),
                query_service_factory.users_service()
            )
        );
    
        query_bus.register::<GetUserByIdQuery, GetUserByIdHandler>(
            GetUserByIdHandler::new(
                ctx.clone(),
                query_service_factory.users_service()
            )
        );
        
        query_bus.register::<GetUserProfileByIdQuery, GetUserProfileByIdHandler>(
            GetUserProfileByIdHandler::new(
                ctx,
                query_service_factory.users_service()
            )
        );
    
        Ok(Application::new(Arc::new(command_bus), Arc::new(query_bus)))
    }
}
