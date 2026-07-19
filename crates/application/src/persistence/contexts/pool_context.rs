use std::any::{ Any };

pub trait PoolContext: Send + Sync {
    fn as_any(&self) -> &dyn Any;
}

impl dyn PoolContext + '_ {
    pub fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}
