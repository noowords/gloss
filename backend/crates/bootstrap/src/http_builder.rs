use std::sync::{ Arc };
use tokio::net::{ TcpListener };

use application::{
    contracts::{ TokenService },
    pipeline::{
        command::{ CommandBus },
        query::{ QueryBus }
    },
    features::{
        auth::{
            commands::{
                send_otp::{ SendOtpCommandHandler },
                verify_otp::{ VerifyOtpCommandHandler },
                refresh_tokens::{ RefreshTokensCommandHandler }
            }
        },
        account::{
            commands::{
                create_profile::{ CreateAccountProfileCommandHandler },
                update_profile::{ UpdateAccountProfileCommandHandler }
            },
            queries::{
                get::{ GetAccountQueryHandler },
                get_profile::{ GetAccountProfileQueryHandler }
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
    contracts::{
        JwtTokenService,
        cqrs::{
            command::{ MySqlCommandContextProvider },
            query::{ MySqlQueryContextProvider }
        }
    },
    features::{
        users::{
            MySqlUserCommandService,
            queries::{
                get_by_id::{ MySqlGetUserByIdQueryService }
            }
        },
        otps::{ MySqlOtpCommandService },
        profiles::{
            MySqlProfileCommandService,
            queries::{
                get_by_user_id::{ MySqlGetProfileByUserIdQueryService }
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
            MySqlAppointmentCommandService,
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
    fn new(
        command_bus: Arc<CommandBus>,
        query_bus: Arc<QueryBus>,
        token_service: Arc<dyn TokenService>
    ) -> Self {
        let state = HttpState::new(command_bus, query_bus, token_service);
        
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

    let token_service = Arc::new(JwtTokenService::new("eUDM8UQkdT0QAGknzfLMKzSF4pvqbou7rYamPjLeGBy".to_string()));

    let user_service = Arc::new(MySqlUserCommandService::default());
    let otp_service = Arc::new(MySqlOtpCommandService::default());
    let profile_service = Arc::new(MySqlProfileCommandService::default());
    let appointment_service = Arc::new(MySqlAppointmentCommandService::default());

    let command_bus = CommandBus::new(command_provider)
        .register(SendOtpCommandHandler::build(otp_service.clone()))
        .register(VerifyOtpCommandHandler::build(otp_service, token_service.clone()))
        .register(RefreshTokensCommandHandler::build(user_service, token_service.clone()))
        .register(CreateAccountProfileCommandHandler::build(profile_service.clone()))
        .register(UpdateAccountProfileCommandHandler::build(profile_service))
        .register(ScheduleAppointmentCommandHandler::build(appointment_service));

    let query_bus = QueryBus::new(query_provider)
        .register(GetAccountQueryHandler::build(
            match database_type.as_str() {
                "mysql" => Arc::new(MySqlGetUserByIdQueryService::default()),
                _ => anyhow::bail!("Unsupported database type: {}", database_type)
            }
        ))
        .register(GetAccountProfileQueryHandler::build(
            match database_type.as_str() {
                "mysql" => Arc::new(MySqlGetProfileByUserIdQueryService::default()),
                _ => anyhow::bail!("Unsupported database type: {}", database_type)
            }
        ))
        .register(GetSpecialistsQueryHandler::build(
            match database_type.as_str() {
                "mysql" => Arc::new(MySqlGetSpecialistsQueryService::default()),
                _ => anyhow::bail!("Unsupported database type: {}", database_type)
            }
        ))
        .register(GetSpecialistByUserIdQueryHandler::build(
            match database_type.as_str() {
                "mysql" => Arc::new(MySqlGetSpecialistByUserIdQueryService::default()),
                _ => anyhow::bail!("Unsupported database type: {}", database_type)
            }
        ))
        .register(GetSpecialistServicesByUserIdQueryHandler::build(
            match database_type.as_str() {
                "mysql" => Arc::new(MySqlGetSpecialistServicesByUserIdQueryService::default()),
                _ => anyhow::bail!("Unsupported database type: {}", database_type)
            }
        ))
        .register(GetAppointmentsQueryHandler::build(
            match database_type.as_str() {
                "mysql" => Arc::new(MySqlGetAppointmentsQueryService::default()),
                _ => anyhow::bail!("Unsupported database type: {}", database_type)
            }
        ))
        .register(GetAppointmentByIdQueryHandler::build(
            match database_type.as_str() {
                "mysql" => Arc::new(MySqlGetAppointmentByIdQueryService::default()),
                _ => anyhow::bail!("Unsupported database type: {}", database_type)
            }
        ));

    Ok(HttpApplication::new(Arc::new(command_bus), Arc::new(query_bus), token_service))
}
