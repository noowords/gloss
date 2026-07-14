use std::sync::{ Arc };

use crate::application::{
    shared::{ CommandBus },
    commands::{
        register_user::{ RegisterUserCommand, RegisterUserHandler },
        create_appointment::{ CreateAppointmentCommand, CreateAppointmentHandler }
    }
};
use crate::domain::shared::{ UnitOfWorkFactory, CommandRepositoryFactory };

pub fn build_command_bus(
    uow_factory: Arc<dyn UnitOfWorkFactory>,
    repository_factory: Arc<dyn CommandRepositoryFactory>
) -> Arc<CommandBus> {
    let mut command_bus = CommandBus::new();

    command_bus.register::<RegisterUserCommand, RegisterUserHandler>(
        RegisterUserHandler::new(
            uow_factory.clone(),
            repository_factory.users_repository(),
            repository_factory.profiles_repository()
        )
    );

    command_bus.register::<CreateAppointmentCommand, CreateAppointmentHandler>(
        CreateAppointmentHandler::new(
            uow_factory,
            repository_factory.appointments_repository()
        )
    );

    Arc::new(command_bus)
}
