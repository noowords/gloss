use async_trait::{ async_trait };

use domain::aggregates::{
    users::user::value_objects::{ UserId },
    users::profile::{ Profile }
};

use crate::common::cqrs::command::{ CommandContext };

#[async_trait]
pub trait UpdateAccountProfileCommandService: Send + Sync + 'static {
    async fn update_profile(&self, ctx: &mut dyn CommandContext, user_id: &UserId, profile: &Profile) -> Result<(), anyhow::Error>;
}
