use std::sync::{ Arc };
use async_trait::{ async_trait };

use crate::domain::{
    shared::{ UnitOfWorkFactory, InfrastructureFactory },
    models::{
        user::{
            User,
            value_objects::{ UserRole, UserPhone }
        }
    }
};

use super::super::super::shared::{ CommandHandler };

use super::{ CreateUserCommand };

pub struct CreateUserHandler {
    uow_factory: Arc<dyn UnitOfWorkFactory>,
    infra_factory: Arc<dyn InfrastructureFactory>
}

impl CreateUserHandler {
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
impl CommandHandler<CreateUserCommand> for CreateUserHandler {
    type Output = ();
    
    async fn handle(&self, cmd: CreateUserCommand) -> Result<Self::Output, Box<dyn std::error::Error + Send + Sync>> {
        let mut uow = self.uow_factory.begin().await?;
        let user_repository = self.infra_factory.user_repository();
        
        let user = User::new(
            None,
            cmd.role
                .as_deref()
                .map(|r| UserRole::try_from(r))
                .transpose()?,
            cmd.phone
                .map(|p| UserPhone::new(p))
                .transpose()?
        );
        
        user_repository.create(&mut *uow, &user).await?;

        uow.commit().await?;
        
        Ok(())
    }
}
