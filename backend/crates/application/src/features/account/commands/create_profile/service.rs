use async_trait::{ async_trait };

use domain::aggregates::users::profile::{ Profile };

use crate::common::cqrs::command::{ CommandContext };

#[async_trait]
pub trait CreateAccountProfileCommandService: Send + Sync + 'static {
    async fn create_profile(&self, ctx: &mut dyn CommandContext, profile: &Profile) -> Result<(), anyhow::Error>;
}
