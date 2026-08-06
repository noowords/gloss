use async_trait::{ async_trait };

use domain::aggregates::{
    user::value_objects::{ UserId },
    profile::{ Profile }
};

use crate::contracts::cqrs::query::{ QueryContext };

#[async_trait]
pub trait GetUserProfileByIdQueryService: Send + Sync {
    async fn get_user_profile_by_id(&self, context: &dyn QueryContext, id: UserId) -> Result<Option<Profile>, anyhow::Error>;
}
