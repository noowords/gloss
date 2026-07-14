use std::sync::{ Arc };
use async_trait::{ async_trait };

use crate::domain::{
    shared::{ UnitOfWorkFactory },
    models::{
        user::{
            User, UserRepository,
            value_objects::{ UserPhone }
        },
        profile::{
            Profile, ProfileRepository
        }
    }
};

use super::super::super::shared::{ CommandHandler };

use super::{ RegisterUserCommand };

pub struct RegisterUserHandler {
    uow_factory: Arc<dyn UnitOfWorkFactory>,
    users_repository: Arc<dyn UserRepository>,
    profiles_repository: Arc<dyn ProfileRepository>
}

impl RegisterUserHandler {
    pub fn new(
        uow_factory: Arc<dyn UnitOfWorkFactory>,
        users_repository: Arc<dyn UserRepository>,
        profiles_repository: Arc<dyn ProfileRepository>
    ) -> Self {
        Self { uow_factory, users_repository, profiles_repository }
    }
}

#[async_trait]
impl CommandHandler<RegisterUserCommand> for RegisterUserHandler {
    type Output = ();
    
    async fn handle(&self, command: RegisterUserCommand) -> Result<Self::Output, Box<dyn std::error::Error + Send + Sync>> {
        let mut uow = self.uow_factory.begin().await?;
        
        let user = User::new(
            None,
            None,
            UserPhone::new(command.phone).map(Some)?
        );
        
        self.users_repository.create(uow.ctx_mut(), &user).await?;

        let profile = Profile::new(
            Some(user.id()),
            command.first_name,
            command.last_name,
            None,
            None
        );

        self.profiles_repository.create(uow.ctx_mut(), &profile).await?;

        uow.commit().await?;
        
        Ok(())
    }
}
