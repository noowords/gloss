use std::any::{ Any };

pub trait QueryContext: Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;
}
