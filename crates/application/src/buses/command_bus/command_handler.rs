use async_trait::{ async_trait };

use crate::contexts::{ TxContext };
use crate::buses::command_bus::{ Command };

#[async_trait]
pub trait CommandHandler<C: Command>: Send + Sync + 'static {
    async fn handle(&self, ctx: &mut dyn TxContext, command: C) -> Result<C::Result, C::Error>;
}
