use async_trait::{ async_trait };

use super::super::super::common::persistence::{ TxContext, RepositoryFactory };

use super::{ Command };

#[async_trait]
pub trait CommandHandler<C: Command>: Send + Sync {
    async fn handle(
        &self,
        ctx: &mut dyn TxContext,
        repository_factory: &dyn RepositoryFactory,
        command: C
    ) -> Result<C::Output, C::Error>;
}
