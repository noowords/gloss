use async_trait::{ async_trait };

use domain::aggregates::user::value_objects::{ UserId };

use crate::contracts::cqrs::query::{ QueryContext };

use super::dtos::{ Service };

#[async_trait]
pub trait GetSpecialistServicesByUserIdQueryService: Send + Sync {
    async fn get_specialist_services_by_user_id(&self, context: &dyn QueryContext, user_id: UserId) -> Result<Vec<Service>, anyhow::Error>;
}
