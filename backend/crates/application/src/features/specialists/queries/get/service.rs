use async_trait::{ async_trait };

use crate::common::cqrs::query::{ QueryContext };

use super::dtos::{ Specialist };

#[async_trait]
pub trait GetSpecialistsQueryService: Send + Sync + 'static {
    async fn get_specialists(&self, context: &dyn QueryContext) -> Result<Vec<Specialist>, anyhow::Error>;
}
