use std::any::{ Any, TypeId };
use std::collections::{ HashMap };
use async_trait::{ async_trait };

pub trait Query: Send + Sync + 'static {}
impl<T: Send + Sync + 'static> Query for T {}

#[async_trait]
pub trait QueryHandler<Q: Query>: Send + Sync {
    type Output: Send + 'static;
    async fn handle(&self, query: Q) -> Result<Self::Output, Box<dyn std::error::Error + Send + Sync>>;
}

type HandlerBox = Box<dyn Any + Send + Sync>;

pub struct QueryBus {
    handlers: HashMap<TypeId, HandlerBox>
}

impl QueryBus {
    pub fn new() -> Self {
        Self { handlers: HashMap::new() }
    }
    
    pub fn register<Q, H>(&mut self, handler: H) -> &mut Self
    where
        Q: Query,
        H: QueryHandler<Q> + 'static,
    {
        let type_id = TypeId::of::<Q>();
        
        let trait_object: Box<dyn QueryHandler<Q, Output = H::Output>> = Box::new(handler);
        
        self.handlers.insert(type_id, Box::new(trait_object));
        self
    }
    
    pub async fn send<Q, R>(&self, query: Q) -> Result<R, Box<dyn std::error::Error + Send + Sync>>
    where
        Q: Query,
        R: Send + 'static,
    {
        let type_id = TypeId::of::<Q>();
        
        let handler_any = self.handlers
            .get(&type_id)
            .ok_or_else(|| format!("No handler registered for query: {:?}", std::any::type_name::<Q>()))?;
        
        let handler = handler_any
            .downcast_ref::<Box<dyn QueryHandler<Q, Output = R>>>()
            .ok_or_else(|| {
                format!(
                    "Type mismatch for query: {:?}",
                    std::any::type_name::<Q>()
                )
            })?;
        
        handler.handle(query).await
    }
}

impl Default for QueryBus {
    fn default() -> Self { Self::new() }
}
