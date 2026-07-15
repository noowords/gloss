use async_trait::{ async_trait };

use crate::domain::models::{
    user::{
        User,
        value_objects::{ UserPhone }
    },
    profile::{ Profile }
};

use super::super::super::common::{
    CommandHandler,
    persistence::{ TxContext, RepositoryFactory }
};

use super::{ RegisterUserCommand };

#[derive(Default)]
pub struct RegisterUserHandler;

impl RegisterUserHandler {
    pub fn new() -> Self {
        Self::default()
    }    
}

#[async_trait]
impl CommandHandler<RegisterUserCommand> for RegisterUserHandler {
    type Output = ();
    type Error = anyhow::Error;
    
    async fn handle(
        &self,
        ctx: &mut dyn TxContext,
        repository_factory: &dyn RepositoryFactory,
        command: RegisterUserCommand
    ) -> Result<Self::Output, Self::Error> {
        let user = User::new(
            None,
            None,
            UserPhone::new(command.phone).map(Some)?
        );
        
        repository_factory.user_repository(ctx)?.create(&user).await?;

        let profile = Profile::new(
            Some(user.id()),
            command.first_name,
            command.last_name,
            None,
            None
        );

        repository_factory.profile_repository(ctx)?.create(&profile).await?;

        Ok(())
    }
}
