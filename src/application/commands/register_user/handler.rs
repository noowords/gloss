use async_trait::{ async_trait };

use crate::domain::models::{
    user::{
        User,
        value_objects::{ UserPhone }
    },
    profile::{ Profile }
};

use super::super::super::common::{
    commands::{ CommandHandler },
    persistence::{ TxContext, RepositoryFactory }
};

use super::{ RegisterUserCommand };

#[derive(Default)]
pub struct RegisterUserHandler;

#[async_trait]
impl CommandHandler<RegisterUserCommand> for RegisterUserHandler {
    async fn handle(
        &self,
        ctx: &mut dyn TxContext,
        repository_factory: &dyn RepositoryFactory,
        command: RegisterUserCommand
    ) -> Result<(), anyhow::Error> {
        let user = User::new(
            None,
            None,
            UserPhone::new(command.phone).map(Some)?
        );
        
        repository_factory.users(ctx)?.create(&user).await?;

        let profile = Profile::new(
            Some(user.id()),
            command.first_name,
            command.last_name,
            None,
            None
        );

        repository_factory.profiles(ctx)?.create(&profile).await?;

        Ok(())
    }
}
