use std::sync::{ Arc };
use async_trait::{ async_trait };

use crate::domain::models::{
    user::{
        User, UserRepository,
        value_objects::{ UserPhone }
    },
    profile::{
        Profile, ProfileRepository
    }
};

use super::super::super::common::{
    CommandHandler,
    persistence::{ TxContext }
};

use super::{ RegisterUserCommand };

pub struct RegisterUserHandler {
    user_repository: Arc<dyn UserRepository>,
    profile_repository: Arc<dyn ProfileRepository>
}

impl RegisterUserHandler {
    pub fn new(
        user_repository: Arc<dyn UserRepository>,
        profile_repository: Arc<dyn ProfileRepository>
    ) -> Self {
        Self { user_repository, profile_repository }
    }
}

#[async_trait]
impl CommandHandler<RegisterUserCommand> for RegisterUserHandler {
    type Output = ();
    type Error = anyhow::Error;
    
    async fn handle(
        &self,
        ctx: &mut dyn TxContext,
        command: RegisterUserCommand
    ) -> Result<Self::Output, Self::Error> {
        let user = User::new(
            None,
            None,
            UserPhone::new(command.phone).map(Some)?
        );
        
        self.user_repository.create(ctx, &user).await?;

        let profile = Profile::new(
            Some(user.id()),
            command.first_name,
            command.last_name,
            None,
            None
        );

        self.profile_repository.create(ctx, &profile).await?;

        Ok(())
    }
}
