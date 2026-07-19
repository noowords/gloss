use std::sync::{ Arc };
use async_trait::{ async_trait };

use super::super::super::super::common::{
    QueryHandler,
    persistence::{ PoolContext }
};

use super::super::{ UsersQueryService };

use super::{ GetUserProfileByIdQuery, GetUserProfileByIdView };

pub struct GetUserProfileByIdHandler {
    ctx: Arc<dyn PoolContext>,
    service: Arc<dyn UsersQueryService>
}

impl GetUserProfileByIdHandler {
    pub fn new(
        ctx: Arc<dyn PoolContext>,
        service: Arc<dyn UsersQueryService>
    ) -> Self {
        Self { ctx, service }
    }
}

#[async_trait]
impl QueryHandler<GetUserProfileByIdQuery> for GetUserProfileByIdHandler {
    type Output = Option<GetUserProfileByIdView>;
    
    async fn handle(&self, query: GetUserProfileByIdQuery) -> Result<Self::Output, Box<dyn std::error::Error + Send + Sync>> {
        self.service.get_profile_by_id(&*self.ctx, query.id)
            .await
            .map_err(|e| e.into())
    }
}
