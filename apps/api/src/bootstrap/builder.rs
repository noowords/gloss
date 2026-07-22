use std::sync::{ Arc };
use sqlx::mysql::{ MySqlPool };

use application::buses::{
    command_bus::{ CommandBus },
    query_bus::{ QueryBus }
};
use application::persistence::{
    commands::{
        RegisterUserCommand,
        ScheduleAppointmentCommand
    },
    command_handlers::{
        RegisterUserCommandHandler,
        ScheduleAppointmentCommandHandler
    }
};
use application::projections::{
    queries::{
        GetUsersQuery,
        GetUserByIdQuery,
        GetUserProfileByIdQuery
    },
    query_handlers::{
        GetUsersQueryHandler,
        GetUserByIdQueryHandler,
        GetUserProfileByIdQueryHandler
    }
};
use infrastructure::contexts::mysql::{ MySqlPoolContext };
use infrastructure::factories::mysql::{ MySqlUnitOfWorkFactory };
use infrastructure::persistence::mysql::command_services::{
    MySqlRegisterUserCommandService,
    MySqlScheduleAppointmentCommandService
};
use infrastructure::projections::mysql::query_services::{
    MySqlGetUsersQueryService,
    MySqlGetUserByIdQueryService,
    MySqlGetUserProfileByIdQueryService
};

use super::{ Application };

pub struct ApplicationBuilder {
    database: Option<(String, String)>
}

impl ApplicationBuilder {
    pub fn new() -> Self {
        Self { database: None }
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

        let mut command_bus = CommandBus::new(uow_factory);

        command_bus.register::<RegisterUserCommand>(
            RegisterUserCommandHandler::build(
                Arc::new(MySqlRegisterUserCommandService::default())
            )
        );

        command_bus.register::<ScheduleAppointmentCommand>(
            ScheduleAppointmentCommandHandler::build(
                Arc::new(MySqlScheduleAppointmentCommandService::default())
            )
        );

        let mut query_bus = QueryBus::new(ctx);

        query_bus.register::<GetUsersQuery>(
            GetUsersQueryHandler::build(
                Arc::new(MySqlGetUsersQueryService::default())
            )
        );
    
        query_bus.register::<GetUserByIdQuery>(
            GetUserByIdQueryHandler::build(
                Arc::new(MySqlGetUserByIdQueryService::default())
            )
        );
        
        query_bus.register::<GetUserProfileByIdQuery>(
            GetUserProfileByIdQueryHandler::build(
                Arc::new(MySqlGetUserProfileByIdQueryService::default())
            )
        );
    
        Ok(Application::new(Arc::new(command_bus), Arc::new(query_bus)))
    }
}
