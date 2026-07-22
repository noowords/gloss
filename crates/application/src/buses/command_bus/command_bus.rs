use std::any::{ Any, TypeId };
use std::collections::{ HashMap };
use std::sync::{ Arc };

use crate::factories::{ UnitOfWorkFactory };
use crate::buses::command_bus::{ Command, CommandHandler };

pub struct CommandBus {
    uow_factory: Arc<dyn UnitOfWorkFactory>,
    handlers: HashMap<TypeId, Box<dyn Any + Send + Sync>>
}

impl CommandBus {
    pub fn new(uow_factory: Arc<dyn UnitOfWorkFactory>) -> Self {
        Self { handlers: HashMap::new(), uow_factory }
    }

    pub fn register<C>(&mut self, handler: C::Handler) -> &mut Self
    where
        C: Command
    {
        let type_id = TypeId::of::<C>();
        
        let trait_object: Box<dyn CommandHandler<C>> = Box::new(handler);
        
        self.handlers.insert(type_id, Box::new(trait_object));
        self
    }
    
    pub async fn send<C>(&self, command: C) -> Result<Result<C::Result, C::Error>, anyhow::Error>
    where
        C: Command
    {
        let type_id = TypeId::of::<C>();
        
        let handler_any = self.handlers.get(&type_id)
            .ok_or_else(|| format!("No handler registered for command: {:?}", std::any::type_name::<C>()))
            .map_err(|e| anyhow::anyhow!(e))?;
        
        let handler = handler_any.downcast_ref::<Box<dyn CommandHandler<C>>>()
            .ok_or_else(|| format!("Type mismatch for command: {:?}", std::any::type_name::<C>()))
            .map_err(|e| anyhow::anyhow!(e))?;

        let mut uow = self.uow_factory.begin()
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        
        let result = handler.handle(uow.ctx_mut(), command).await;

        uow.commit()
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        Ok(result)
    }
}
