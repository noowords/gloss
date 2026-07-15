use std::any::{ Any, TypeId };
use std::collections::{ HashMap };
use async_trait::{ async_trait };

pub trait Command: Send + Sync + 'static {}
impl<T: Send + Sync + 'static> Command for T {}

#[async_trait]
pub trait CommandHandler<C: Command>: Send + Sync {
    type Output: Send + 'static;
    async fn handle(&self, command: C) -> Result<Self::Output, Box<dyn std::error::Error + Send + Sync>>;
}

type HandlerBox = Box<dyn Any + Send + Sync>;

pub struct CommandBus {
    handlers: HashMap<TypeId, HandlerBox>
}

impl CommandBus {
    pub fn new() -> Self {
        Self { handlers: HashMap::new() }
    }
    
    pub fn register<C, H>(&mut self, handler: H) -> &mut Self
    where
        C: Command,
        H: CommandHandler<C> + 'static,
    {
        let type_id = TypeId::of::<C>();
        
        let trait_object: Box<dyn CommandHandler<C, Output = H::Output>> = Box::new(handler);
        
        self.handlers.insert(type_id, Box::new(trait_object));
        self
    }
    
    pub async fn send<C, R>(&self, command: C) -> Result<R, Box<dyn std::error::Error + Send + Sync>>
    where
        C: Command,
        R: Send + 'static,
    {
        let type_id = TypeId::of::<C>();
        
        let handler_any = self.handlers
            .get(&type_id)
            .ok_or_else(|| format!("No handler registered for command: {:?}", std::any::type_name::<C>()))?;
        
        let handler = handler_any
            .downcast_ref::<Box<dyn CommandHandler<C, Output = R>>>()
            .ok_or_else(|| {
                format!(
                    "Type mismatch for command: {:?}. Чекайте совпадение возвращаемого типа (Output) в send::<C, R>()!",
                    std::any::type_name::<C>()
                )
            })?;
        
        handler.handle(command).await
    }
}

impl Default for CommandBus {
    fn default() -> Self { Self::new() }
}
