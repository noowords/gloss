use async_trait::{ async_trait };

use super::{ Command, CommandContext };

#[async_trait]
pub trait CommandHandler<C: Command>: Send + Sync + 'static {
    async fn handle(&self, context: &mut dyn CommandContext, command: C) -> Result<C::Result, C::Error>;
}
