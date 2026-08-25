use std::sync::{ Arc };
use tokio::net::{ TcpListener };

use application::{
    pipeline::{
        command::{ CommandBus },
        query::{ QueryBus }
    },
    features::{
        users::{
            queries::{
                get::{ GetUsersQueryHandler },
                get_by_id::{ GetUserByIdQueryHandler },
                get_profile_by_id::{ GetUserProfileByIdQueryHandler }
            }
        },
        specialists::{
            queries::{
                get::{ GetSpecialistsQueryHandler },
                get_by_user_id::{ GetSpecialistByUserIdQueryHandler },
                get_services_by_user_id::{ GetSpecialistServicesByUserIdQueryHandler }
            }
        },
        appointments::{
            commands::{
                schedule::{ ScheduleAppointmentCommandHandler }
            },
            queries::{
                get::{ GetAppointmentsQueryHandler },
                get_by_id::{ GetAppointmentByIdQueryHandler }
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
            queries::{
                get::{ MySqlGetUsersQueryService },
                get_by_id::{ MySqlGetUserByIdQueryService },
                get_profile_by_id::{ MySqlGetUserProfileByIdQueryService }
            }
        },
        specialists::{
            queries::{
                get::{ MySqlGetSpecialistsQueryService },
                get_by_user_id::{ MySqlGetSpecialistByUserIdQueryService },
                get_services_by_user_id::{ MySqlGetSpecialistServicesByUserIdQueryService }
            }
        },
        appointments::{
            commands::{
                schedule::{ MySqlScheduleAppointmentCommandService }
            },
            queries::{
                get::{ MySqlGetAppointmentsQueryService },
                get_by_id::{ MySqlGetAppointmentByIdQueryService }
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
        let api_port = std::env::var("API_PORT")?.parse::<u16>()?;
        
        let router = create_http_router(self.state);
    
        let listener = TcpListener::bind(format!("0.0.0.0:{}", api_port)).await?;
    
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

    command_bus.register(
        ScheduleAppointmentCommandHandler::build(
            match database_type.as_str() {
                "mysql" => Arc::new(MySqlScheduleAppointmentCommandService::default()),
                _ => anyhow::bail!("Unsupported database type: {}", database_type)
            }
        )
    );

    let mut query_bus = QueryBus::new(query_provider);

    query_bus.register(
        GetUsersQueryHandler::build(
            match database_type.as_str() {
                "mysql" => Arc::new(MySqlGetUsersQueryService::default()),
                _ => anyhow::bail!("Unsupported database type: {}", database_type)
            }
        )
    );

    query_bus.register(
        GetUserByIdQueryHandler::build(
            match database_type.as_str() {
                "mysql" => Arc::new(MySqlGetUserByIdQueryService::default()),
                _ => anyhow::bail!("Unsupported database type: {}", database_type)
            }
        )
    );
    
    query_bus.register(
        GetUserProfileByIdQueryHandler::build(
            match database_type.as_str() {
                "mysql" => Arc::new(MySqlGetUserProfileByIdQueryService::default()),
                _ => anyhow::bail!("Unsupported database type: {}", database_type)
            }
        )
    );
    
    query_bus.register(
        GetSpecialistsQueryHandler::build(
            match database_type.as_str() {
                "mysql" => Arc::new(MySqlGetSpecialistsQueryService::default()),
                _ => anyhow::bail!("Unsupported database type: {}", database_type)
            }
        )
    );

    query_bus.register(
        GetSpecialistByUserIdQueryHandler::build(
            match database_type.as_str() {
                "mysql" => Arc::new(MySqlGetSpecialistByUserIdQueryService::default()),
                _ => anyhow::bail!("Unsupported database type: {}", database_type)
            }
        )
    );
    
    query_bus.register(
        GetSpecialistServicesByUserIdQueryHandler::build(
            match database_type.as_str() {
                "mysql" => Arc::new(MySqlGetSpecialistServicesByUserIdQueryService::default()),
                _ => anyhow::bail!("Unsupported database type: {}", database_type)
            }
        )
    );
    
    query_bus.register(
        GetAppointmentsQueryHandler::build(
            match database_type.as_str() {
                "mysql" => Arc::new(MySqlGetAppointmentsQueryService::default()),
                _ => anyhow::bail!("Unsupported database type: {}", database_type)
            }
        )
    );
    
    query_bus.register(
        GetAppointmentByIdQueryHandler::build(
            match database_type.as_str() {
                "mysql" => Arc::new(MySqlGetAppointmentByIdQueryService::default()),
                _ => anyhow::bail!("Unsupported database type: {}", database_type)
            }
        )
    );

    Ok(HttpApplication::new(Arc::new(command_bus), Arc::new(query_bus)))
}
