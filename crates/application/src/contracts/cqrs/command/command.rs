pub trait Command: Clone + Send + Sync + 'static {
    type Result: Send + Sync + 'static;
    type Error: Send + Sync + 'static;
}
