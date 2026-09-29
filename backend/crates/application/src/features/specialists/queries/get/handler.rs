use async_trait::{ async_trait };

use crate::common::cqrs::query::{ Query, QueryHandler, QueryContext };
use super::{ GetSpecialistsQuery, GetSpecialistsQueryService };

pub struct GetSpecialistsQueryHandler<S> {
    service: S
}

impl<S> GetSpecialistsQueryHandler<S>
where
    S: GetSpecialistsQueryService
{
    pub fn build(service: S) -> Self {
        Self { service }
    }
}

#[async_trait]
impl<S> QueryHandler<GetSpecialistsQuery> for GetSpecialistsQueryHandler<S>
where
    S: GetSpecialistsQueryService
{
    async fn handle(&self, context: &dyn QueryContext, _query: GetSpecialistsQuery) -> Result<
        <GetSpecialistsQuery as Query>::View,
        <GetSpecialistsQuery as Query>::Error
    > {
        self.service.get_specialists(context).await
    }
}
