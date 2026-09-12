use async_trait::{ async_trait };

use domain::aggregates::user::value_objects::{ UserId };

use crate::contracts::cqrs::query::{ QueryContext };

use super::dtos::{ Profile };

#[async_trait]
pub trait GetAccountProfileQueryService: Send + Sync {
    async fn get_profile_by_user_id(&self, context: &dyn QueryContext, user_id: UserId) -> Result<Option<Profile>, anyhow::Error>;
}
