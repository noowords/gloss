use std::sync::{ Arc };

use crate::application::{
    common::{ QueryServiceFactory, QueryBus },
    queries::{
        users::{
            get::{ GetUsersQuery, GetUsersHandler },
            get_by_id::{ GetUserByIdQuery, GetUserByIdHandler },
            get_profile_by_id::{ GetUserProfileByIdQuery, GetUserProfileByIdHandler }
        }
    }
};
use crate::domain::common::{ PoolContext };

pub fn build_query_bus(
    ctx: Arc<dyn PoolContext>,
    service_factory: Arc<dyn QueryServiceFactory>
) -> Arc<QueryBus> {
    let mut command_bus = QueryBus::new();
    
    command_bus.register::<GetUsersQuery, GetUsersHandler>(
        GetUsersHandler::new(
            ctx.clone(),
            service_factory.users_service()
        )
    );

    command_bus.register::<GetUserByIdQuery, GetUserByIdHandler>(
        GetUserByIdHandler::new(
            ctx.clone(),
            service_factory.users_service()
        )
    );
    
    command_bus.register::<GetUserProfileByIdQuery, GetUserProfileByIdHandler>(
        GetUserProfileByIdHandler::new(
            ctx,
            service_factory.users_service()
        )
    );

    Arc::new(command_bus)
}
