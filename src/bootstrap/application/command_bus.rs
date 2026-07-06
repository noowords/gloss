use std::sync::{ Arc };

use crate::application::{
    shared::{ CommandBus },
    commands::{
        create_user::{ CreateUserCommand, CreateUserHandler },
        create_appointment::{ CreateAppointmentCommand, CreateAppointmentHandler }
    }
};
use crate::domain::shared::{ UnitOfWorkFactory, InfrastructureFactory };

pub fn build_command_bus(
    uow_factory: Arc<dyn UnitOfWorkFactory>,
    infra_factory: Arc<dyn InfrastructureFactory>
) -> Arc<CommandBus> {
    let mut command_bus = CommandBus::new();

    command_bus.register::<CreateUserCommand, _>(
        CreateUserHandler::new(
            uow_factory.clone(),
            infra_factory.clone(),
        )
    );

    command_bus.register::<CreateAppointmentCommand, _>(
        CreateAppointmentHandler::new(
            uow_factory.clone(),
            infra_factory.clone(),
        )
    );

    Arc::new(command_bus)
}
