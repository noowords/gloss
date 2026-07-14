use std::sync::{ Arc };

use crate::application::shared::{ QueryServiceFactory };
use crate::domain::shared::{ PoolContext, UnitOfWorkFactory, CommandRepositoryFactory };

#[derive(Clone)]
pub struct InfraState {
    pub ctx: Arc<dyn PoolContext>,
    pub uow_factory: Arc<dyn UnitOfWorkFactory>,
    pub cr_factory: Arc<dyn CommandRepositoryFactory>,
    pub qs_factory: Arc<dyn QueryServiceFactory>
}

impl InfraState {
    pub fn new(
        ctx: Arc<dyn PoolContext>,
        uow_factory: Arc<dyn UnitOfWorkFactory>,
        cr_factory: Arc<dyn CommandRepositoryFactory>,
        qs_factory: Arc<dyn QueryServiceFactory>
    ) -> Self {
        Self {
            ctx,
            uow_factory,
            cr_factory,
            qs_factory
        }
    }
}
