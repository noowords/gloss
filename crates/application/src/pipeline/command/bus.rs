use std::any::{ Any, TypeId };
use std::collections::{ HashMap };
use std::sync::{ Arc };

use crate::contracts::cqrs::command::{ Command, CommandHandler, CommandContextProvider };

pub struct CommandBus {
    provider: Arc<dyn CommandContextProvider>,
    handlers: HashMap<TypeId, Box<dyn Any + Send + Sync>>
}

impl CommandBus {
    pub fn new(provider: Arc<dyn CommandContextProvider>) -> Self {
        Self { provider, handlers: HashMap::new() }
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

    pub async fn dispatch<C>(&self, command: C) -> Result<Result<(), C::Error>, anyhow::Error>
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

        let mut context = self.provider.provide_context()
            .await
            .map_err(|e| anyhow::anyhow!(e))?;

        match handler.handle(&mut *context, command).await {
            Ok(result) => {
                context.commit()
                    .await
                    .map_err(|e| anyhow::anyhow!(e))?;
                
                Ok(Ok(result))
            },
            Err(e) => {
                context.rollback()
                    .await
                    .map_err(|e| anyhow::anyhow!(e))?;

                Ok(Err(e))
            }
        }
    }
}
