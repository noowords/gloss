use async_trait::{ async_trait };

use domain::user::{
    profile::{ Profile },
    value_objects::{ UserId }
};

use crate::contexts::{ PoolContext };

#[async_trait]
pub trait GetUserProfileByIdQueryService: Send + Sync {
    async fn get_user_profile_by_id(&self, ctx: &dyn PoolContext, id: UserId) -> Result<Option<Profile>, anyhow::Error>;
}
