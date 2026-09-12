use async_trait::{ async_trait };

use domain::aggregates::{
    user::value_objects::{ UserId },
    profile::{ Profile }
};

use crate::contracts::cqrs::command::{ CommandContext };

#[async_trait]
pub trait UpdateAccountProfileCommandService: Send + Sync {
    async fn update_profile(&self, ctx: &mut dyn CommandContext, user_id: &UserId, profile: &Profile) -> Result<(), anyhow::Error>;
}
