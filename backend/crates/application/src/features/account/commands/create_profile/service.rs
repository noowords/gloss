use async_trait::{ async_trait };

use domain::aggregates::profile::{ Profile };

use crate::contracts::cqrs::command::{ CommandContext };

#[async_trait]
pub trait CreateAccountProfileCommandService: Send + Sync {
    async fn create_profile(&self, ctx: &mut dyn CommandContext, profile: &Profile) -> Result<(), anyhow::Error>;
}
