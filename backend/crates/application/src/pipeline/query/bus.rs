use std::any::{ Any, TypeId };
use std::collections::{ HashMap };
use std::sync::{ Arc };

use crate::contracts::cqrs::query::{ Query, QueryHandler, QueryContextProvider };

pub struct QueryBus {
    provider: Arc<dyn QueryContextProvider>,
    handlers: HashMap<TypeId, Box<dyn Any + Send + Sync>>
}

impl QueryBus {
    pub fn new(provider: Arc<dyn QueryContextProvider>) -> Self {
        Self { provider, handlers: HashMap::new() }
    }

    pub fn register<Q, H>(&mut self, handler: H) -> &mut Self
    where
        Q: Query,
        H: QueryHandler<Q>
    {
        let type_id = TypeId::of::<Q>();
        let trait_object: Box<dyn QueryHandler<Q>> = Box::new(handler);

        self.handlers.insert(type_id, Box::new(trait_object));
        
        self
    }

    pub async fn dispatch<Q>(&self, query: Q) -> Result<Result<Q::View, Q::Error>, anyhow::Error>
    where
        Q: Query
    {
        let type_id = TypeId::of::<Q>();

        let handler_any = self.handlers.get(&type_id)
            .ok_or_else(|| format!("No handler registered for query: {:?}", std::any::type_name::<Q>()))
            .map_err(|e| anyhow::anyhow!(e))?;

        let handler = handler_any.downcast_ref::<Box<dyn QueryHandler<Q>>>()
            .ok_or_else(|| format!("Type mismatch for query: {:?}", std::any::type_name::<Q>()))
            .map_err(|e| anyhow::anyhow!(e))?;

        let context = self.provider.provide_context();

        Ok(handler.handle(&*context, query).await)
    }
}
