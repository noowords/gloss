use async_trait::{ async_trait };

use domain::aggregates::user::value_objects::{ UserId };

use crate::contracts::cqrs::query::{ QueryContext };

use super::dtos::{ Specialist };

#[async_trait]
pub trait GetSpecialistByUserIdQueryService: Send + Sync {
    async fn get_specialist_by_user_id(&self, context: &dyn QueryContext, user_id: UserId) -> Result<Option<Specialist>, anyhow::Error>;
}
