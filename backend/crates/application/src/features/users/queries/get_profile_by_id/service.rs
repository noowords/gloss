use async_trait::{ async_trait };

use domain::aggregates::user::value_objects::{ UserId };

use crate::contracts::cqrs::query::{ QueryContext };

use super::dtos::{ Profile };

#[async_trait]
pub trait GetUserProfileByIdQueryService: Send + Sync {
    async fn get_user_profile_by_id(&self, context: &dyn QueryContext, id: UserId) -> Result<Option<Profile>, anyhow::Error>;
}
