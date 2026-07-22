use crate::{ QueryContext };

pub trait QueryProvider: Send + Sync {
    fn provide_context(&self) -> Box<dyn QueryContext>;
}
