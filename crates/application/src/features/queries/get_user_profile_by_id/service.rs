use async_trait::{ async_trait };

use domain::aggregates::user::{
    profile::{ Profile },
    value_objects::{ UserId }
};

use crate::interfaces::query::{ QueryContext };

#[async_trait]
pub trait GetUserProfileByIdQueryService: Send + Sync {
    async fn get_user_profile_by_id(&self, context: &dyn QueryContext, id: UserId) -> Result<Option<Profile>, anyhow::Error>;
}
