use async_trait::{ async_trait };

use crate::contracts::cqrs::query::{ QueryContext };

use super::dtos::{ Specialist };

#[async_trait]
pub trait GetSpecialistsQueryService: Send + Sync {
    async fn get_specialists(&self, context: &dyn QueryContext) -> Result<Vec<Specialist>, anyhow::Error>;
}
