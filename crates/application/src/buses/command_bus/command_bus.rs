use std::sync::Arc;

use super::super::super::persistence::factories::{RepositoryFactory, UnitOfWorkFactory};

use super::{Command, CommandHandler};

pub struct CommandBus {
    uow_factory: Arc<dyn UnitOfWorkFactory>,
    repository_factory: Arc<dyn RepositoryFactory>,
}

impl CommandBus {
    pub fn new(
        uow_factory: Arc<dyn UnitOfWorkFactory>,
        repository_factory: Arc<dyn RepositoryFactory>,
    ) -> Self {
        Self {
            uow_factory,
            repository_factory,
        }
    }

    pub async fn send<C>(&self, command: C) -> Result<C::Output, C::Error>
    where
        C: Command,
    {
        let mut uow = self
            .uow_factory
            .begin()
            .await
            .map_err(|e| C::Error::from(e))?;

        let handler = C::Handler::default();

        let result = handler
            .handle(uow.ctx_mut(), &*self.repository_factory, command)
            .await?;

        uow.commit().await.map_err(|e| C::Error::from(e))?;

        Ok(result)
    }
}
