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
        command::{ MySqlCommandProvider },
        query::{ MySqlQueryProvider }
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
    let pool = MySqlConnection::connect("mysql://root:root@localhost:3306/gloss").await?;
    
    let command_provider = Arc::new(MySqlCommandProvider::new(pool.clone()));
    let query_provider = Arc::new(MySqlQueryProvider::new(pool));

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

    Ok(HttpApplication::new(Arc::new(command_bus), Arc::new(query_bus)))
}
