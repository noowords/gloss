use std::sync::{ Arc };
use tokio::net::{ TcpListener };

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
    contracts::cqrs::{
        command::{ MySqlCommandContextProvider },
        query::{ MySqlQueryContextProvider }
    },
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
use infrastructure::persistence::mysql::{ MySqlConnection };
use presentation::http::{ HttpState, create_http_router, serve_http };

pub struct HttpApplication {
    state: HttpState
}

impl HttpApplication {
    fn new(command_bus: Arc<CommandBus>, query_bus: Arc<QueryBus>) -> Self {
        let state = HttpState::new(command_bus, query_bus);
        
        Self { state }
    }
    
    pub async fn run(self) -> Result<(), anyhow::Error> {
        let router = create_http_router(self.state);
    
        let listener = TcpListener::bind("localhost:3000").await?;
    
        serve_http(listener, router).await
    }
}

pub async fn build_http() -> Result<HttpApplication, anyhow::Error> {
    let database_type = std::env::var("DATABASE_TYPE")?;
    let database_url = std::env::var("DATABASE_URL")?;
    
    let pool = match database_type.as_str() {
        "mysql" => MySqlConnection::connect(&database_url).await?,
        _ => anyhow::bail!("Unsupported database type: {}", database_type)
    };

    let command_provider = match database_type.as_str() {
        "mysql" => Arc::new(MySqlCommandContextProvider::new(pool.clone())),
        _ => anyhow::bail!("Unsupported database type: {}", database_type)
    };
    let query_provider = match database_type.as_str() {
        "mysql" => Arc::new(MySqlQueryContextProvider::new(pool)),
        _ => anyhow::bail!("Unsupported database type: {}", database_type)
    };

    let mut command_bus = CommandBus::new(command_provider);

    let register_user_command_service = match database_type.as_str() {
        "mysql" => Arc::new(MySqlRegisterUserCommandService::default()),
        _ => anyhow::bail!("Unsupported database type: {}", database_type)
    };

    let schedule_appointment_command_service = match database_type.as_str() {
        "mysql" => Arc::new(MySqlScheduleAppointmentCommandService::default()),
        _ => anyhow::bail!("Unsupported database type: {}", database_type)
    };

    command_bus.register::<RegisterUserCommand>(
        RegisterUserCommandHandler::build(
            register_user_command_service
        )
    );

    command_bus.register::<ScheduleAppointmentCommand>(
        ScheduleAppointmentCommandHandler::build(
            schedule_appointment_command_service
        )
    );

    let mut query_bus = QueryBus::new(query_provider);

    let get_users_query_service = match database_type.as_str() {
        "mysql" => Arc::new(MySqlGetUsersQueryService::default()),
        _ => anyhow::bail!("Unsupported database type: {}", database_type)
    };

    let get_user_by_id_query_service = match database_type.as_str() {
        "mysql" => Arc::new(MySqlGetUserByIdQueryService::default()),
        _ => anyhow::bail!("Unsupported database type: {}", database_type)
    };

    let get_user_profile_by_id_query_service = match database_type.as_str() {
        "mysql" => Arc::new(MySqlGetUserProfileByIdQueryService::default()),
        _ => anyhow::bail!("Unsupported database type: {}", database_type)
    };

    query_bus.register::<GetUsersQuery>(
        GetUsersQueryHandler::build(
            get_users_query_service
        )
    );

    query_bus.register::<GetUserByIdQuery>(
        GetUserByIdQueryHandler::build(
            get_user_by_id_query_service
        )
    );
    
    query_bus.register::<GetUserProfileByIdQuery>(
        GetUserProfileByIdQueryHandler::build(
            get_user_profile_by_id_query_service
        )
    );

    Ok(HttpApplication::new(Arc::new(command_bus), Arc::new(query_bus)))
}
