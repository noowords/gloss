use std::sync::{ Arc };
use async_trait::{ async_trait };

use crate::domain::shared::{ UnitOfWorkFactory, InfrastructureFactory };

use super::super::super::shared::{ QueryHandler };

use super::{ GetUserByIdQuery };

pub struct GetUserByIdHandler {
    uow_factory: Arc<dyn UnitOfWorkFactory>,
    infra_factory: Arc<dyn InfrastructureFactory>
}

impl GetUserByIdHandler {
    pub fn new(
        uow_factory: Arc<dyn UnitOfWorkFactory>,
        infra_factory: Arc<dyn InfrastructureFactory>
    ) -> Self {
        Self {
            uow_factory,
            infra_factory
        }
    }
}

#[async_trait]
impl QueryHandler<GetUserByIdQuery> for GetUserByIdHandler {
    type Output = ();
    
    async fn handle(&self, query: GetUserByIdQuery) -> Result<Self::Output, Box<dyn std::error::Error + Send + Sync>> {
        let mut uow = self.uow_factory.begin().await?;
        let user_repository = self.infra_factory.user_repository();
        
        let user = user_repository.get_by_id(&mut *uow, query.id.into()).await?;

        uow.commit().await?;

        Ok(())
    }
}
