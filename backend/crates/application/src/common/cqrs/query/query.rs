pub trait Query: Clone + Send + Sync + 'static {
    type View: Send + Sync + 'static;
    type Error: Send + Sync + 'static;
}
