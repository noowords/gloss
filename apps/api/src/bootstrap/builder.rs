use std::sync::{ Arc };

use application::{
    pipeline::{
        command::{ CommandBus },
        query::{ QueryBus }
    },
    features::{
        users::{
            commands::{
                register_user::{ RegisterUserCommand, RegisterUserCommandHandler }
            },
            queries::{
                get_users::{ GetUsersQuery, GetUsersQueryHandler },
                get_user_by_id::{ GetUserByIdQuery, GetUserByIdQueryHandler },
                get_user_profile_by_id::{ GetUserProfileByIdQuery, GetUserProfileByIdQueryHandler }
            }
        },
        appointments::{
            commands::{
                schedule_appointment::{ ScheduleAppointmentCommand, ScheduleAppointmentCommandHandler }
            }
        }
    }
};
use infrastructure::persistence::mysql::{
    MySqlDatabaseProvider,
    features::{
        users::{
            commands::{
                register_user::{ MySqlRegisterUserCommandService }
            },
            queries::{
                get_users::{ MySqlGetUsersQueryService },
                get_user_by_id::{ MySqlGetUserByIdQueryService },
                get_user_profile_by_id::{ MySqlGetUserProfileByIdQueryService }
            }
        },
        appointments::{
            commands::{
                schedule_appointment::{ MySqlScheduleAppointmentCommandService }
            }
        }
    }
};

use super::{ Application };

pub enum Database {
    MySql(String)
}

pub struct ApplicationBuilder {
    database: Option<Database>
}

impl ApplicationBuilder {
    pub fn new() -> Self {
        Self { database: None }
    }

    pub fn with_database(mut self, database: Database) -> Self {
        self.database = Some(database);
        self
    }

    pub async fn build(self) -> Result<Application, anyhow::Error> {
        let database = self.database
            .ok_or_else(|| anyhow::anyhow!("Database is not configured"))?;

        let database_provider = match database {
            Database::MySql(url) => MySqlDatabaseProvider::connect(&url).await?,
        };
        
        let command_provider = database_provider.command_provider();
        let query_provider = database_provider.query_provider();

        let mut command_bus = CommandBus::new(command_provider);

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

        let mut query_bus = QueryBus::new(query_provider);

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
