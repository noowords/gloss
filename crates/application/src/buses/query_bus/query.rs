pub trait Query: Send + Sync + 'static {}
impl<T: Send + Sync + 'static> Query for T {}
