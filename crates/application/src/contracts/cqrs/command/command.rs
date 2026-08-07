use super::{ CommandHandler };

pub trait Command: Clone + Send + Sync + 'static {
    type Error: Send + Sync + 'static;

    type Handler: CommandHandler<Self> + Send + Sync + 'static
    where
        Self: Sized;
}
