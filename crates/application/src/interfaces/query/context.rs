use std::any::{ Any };

pub trait QueryContext: Send + Sync {
    fn as_any(&self) -> &dyn Any;
}
