use async_trait::{ async_trait };

use domain::aggregates::{
    user::{ User },
    profile::{ Profile }
};

use crate::contracts::cqrs::command::{ CommandContext };

#[async_trait]
pub trait RegisterUserCommandService: Send + Sync {
    async fn save_user(&self, ctx: &mut dyn CommandContext, user: &User) -> Result<(), anyhow::Error>;

    async fn save_profile(&self, ctx: &mut dyn CommandContext, profile: &Profile) -> Result<(), anyhow::Error>;
}
