use std::sync::{ Arc };

use application::common::{
    commands::{ CommandBus },
    persistence::{ UnitOfWorkFactory, RepositoryFactory }
};

pub fn build_command_bus(
    uow_factory: Arc<dyn UnitOfWorkFactory>,
    repository_factory: Arc<dyn RepositoryFactory>
) -> Arc<CommandBus> {
    Arc::new(CommandBus::new(uow_factory, repository_factory))
}
