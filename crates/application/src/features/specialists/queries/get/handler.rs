use async_trait::{ async_trait };
use std::sync::{ Arc };

use crate::contracts::cqrs::query::{ Query, QueryHandler, QueryContext };
use super::{ GetSpecialistsQuery, GetSpecialistsQueryService };

pub struct GetSpecialistsQueryHandler {
    service: Arc<dyn GetSpecialistsQueryService>
}

impl GetSpecialistsQueryHandler {
    pub fn build(service: Arc<dyn GetSpecialistsQueryService>) -> Self {
        Self { service }
    }
}

#[async_trait]
impl QueryHandler<GetSpecialistsQuery> for GetSpecialistsQueryHandler {
    async fn handle(&self, context: &dyn QueryContext, _query: GetSpecialistsQuery) -> Result<
        <GetSpecialistsQuery as Query>::View,
        <GetSpecialistsQuery as Query>::Error
    > {
        self.service.get_specialists(context).await
    }
}
