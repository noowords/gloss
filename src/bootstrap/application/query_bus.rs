use std::sync::{ Arc };

use crate::application::{
    shared::{ QueryBus },
    queries::{
        get_user_by_id::{ GetUserByIdQuery, GetUserByIdHandler }
    }
};
use crate::domain::shared::{ UnitOfWorkFactory, InfrastructureFactory };

pub fn build_query_bus(
    uow_factory: Arc<dyn UnitOfWorkFactory>,
    infra_factory: Arc<dyn InfrastructureFactory>
) -> Arc<QueryBus> {
    let mut command_bus = QueryBus::new();

    command_bus.register::<GetUserByIdQuery, _>(
        GetUserByIdHandler::new(
            uow_factory.clone(),
            infra_factory.clone()
        )
    );

    Arc::new(command_bus)
}
