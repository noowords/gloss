use crate::buses::query_bus::{ QueryHandler };

pub trait Query: Clone + Send + Sync + 'static {
    type Result: Send + Sync + 'static;
    type Error: Send + Sync + 'static;
    
    type Handler: QueryHandler<Self> + Send + Sync + 'static
    where
        Self: Sized;
}
