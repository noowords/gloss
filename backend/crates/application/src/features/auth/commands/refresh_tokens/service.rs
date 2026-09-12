use async_trait::{ async_trait };

use domain::aggregates::user::{
    User,
    value_objects::{ UserId }
};

use crate::contracts::cqrs::command::{ CommandContext };

#[async_trait]
pub trait RefreshTokensCommandService: Send + Sync {
    async fn get_user_by_id(&self, ctx: &mut dyn CommandContext, user_id: &UserId) -> Result<Option<User>, anyhow::Error>;

    async fn check_profile_exists(&self, ctx: &mut dyn CommandContext, user_id: &UserId) -> Result<bool, anyhow::Error>;
}
