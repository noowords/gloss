use super::{ QueryContext };

pub trait QueryContextProvider: Send + Sync {
    fn provide_context(&self) -> Box<dyn QueryContext>;
}
