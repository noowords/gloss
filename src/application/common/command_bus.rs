use std::any::{ Any, TypeId };
use std::collections::{ HashMap };
use std::sync::{ Arc };
use async_trait::{ async_trait };

use super::super::common::persistence::{ TxContext, UnitOfWorkFactory, RepositoryFactory };

pub trait Command: Send + Sync + 'static {}
impl<T: Send + Sync + 'static> Command for T {}

#[async_trait]
pub trait CommandHandler<C: Command>: Send + Sync {
    type Output: Send + 'static;
    type Error: Send + 'static;
    
    async fn handle(
        &self,
        ctx: &mut dyn TxContext,
        repository_factory: &dyn RepositoryFactory,
        command: C
    ) -> Result<Self::Output, Self::Error>;
}

pub struct CommandBus {
    handlers: HashMap<TypeId, Box<dyn Any + Send + Sync>>,
    uow_factory: Arc<dyn UnitOfWorkFactory>,
    repository_factory: Arc<dyn RepositoryFactory>
}

impl CommandBus {
    pub fn new(uow_factory: Arc<dyn UnitOfWorkFactory>, repository_factory: Arc<dyn RepositoryFactory>) -> Self {
        Self { handlers: HashMap::new(), uow_factory, repository_factory }
    }
    
    pub fn register<C, H>(&mut self) -> &mut Self
    where
        C: Command,
        H: CommandHandler<C> + Default + 'static,
    {
        let type_id = TypeId::of::<C>();
        
        let trait_object: Box<dyn CommandHandler<C, Output = H::Output, Error = H::Error>> = Box::new(H::default());
        
        self.handlers.insert(type_id, Box::new(trait_object));
        self
    }
    
    pub async fn send<C, R, E>(&self, command: C) -> Result<R, anyhow::Error>
    where
        C: Command,
        R: Send + 'static,
        E: Into<anyhow::Error> + Send + 'static
    {
        let type_id = TypeId::of::<C>();
        
        let handler_any = self.handlers
            .get(&type_id)
            .ok_or_else(|| anyhow::anyhow!("No handler registered for command: {:?}", std::any::type_name::<C>()))?;
                
        let handler = handler_any
            .downcast_ref::<Box<dyn CommandHandler<C, Output = R, Error = E>>>()
            .ok_or_else(|| anyhow::anyhow!("Type mismatch for command: {:?}", std::any::type_name::<C>()))?;

        let mut uow = self.uow_factory.begin().await?;
        
        let result = handler.handle(uow.ctx_mut(), &*self.repository_factory, command)
            .await
            .map_err(|e| e.into())?;

        uow.commit().await?;

        Ok(result)
    }
}
