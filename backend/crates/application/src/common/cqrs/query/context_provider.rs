use super::{ QueryContext };

pub trait QueryContextProvider: Send + Sync + 'static {
    fn provide_context(&self) -> Box<dyn QueryContext>;
}
