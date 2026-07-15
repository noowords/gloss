use super::{ CommandHandler };

pub trait Command: Send + Sync + 'static {
    type Output: Send + 'static;
    type Error: From<anyhow::Error> + Send + 'static;
    type Handler: CommandHandler<Self> + Default + 'static
    where Self: Sized;
}
