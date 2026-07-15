use std::sync::{ Arc };

use crate::application::{
    common::{
        CommandBus,
        persistence::{ UnitOfWorkFactory, RepositoryFactory }
    },
    commands::{
        register_user::{ RegisterUserCommand, RegisterUserHandler },
        create_appointment::{ CreateAppointmentCommand, CreateAppointmentHandler }
    }
};

pub fn build_command_bus(
    uow_factory: Arc<dyn UnitOfWorkFactory>,
    repository_factory: Arc<dyn RepositoryFactory>
) -> Arc<CommandBus> {
    let mut command_bus = CommandBus::new(uow_factory);

    command_bus.register::<RegisterUserCommand, RegisterUserHandler>(
        RegisterUserHandler::new(
            repository_factory.user_repository(),
            repository_factory.profile_repository()
        )
    );

    command_bus.register::<CreateAppointmentCommand, CreateAppointmentHandler>(
        CreateAppointmentHandler::new(
            repository_factory.appointment_repository()
        )
    );

    Arc::new(command_bus)
}
