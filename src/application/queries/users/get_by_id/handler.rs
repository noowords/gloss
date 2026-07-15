use std::sync::{ Arc };
use async_trait::{ async_trait };

use super::super::super::super::common::{
    QueryHandler,
    persistence::{ PoolContext }
};

use super::super::{ UsersQueryService };

use super::{ GetUserByIdQuery, GetUserByIdView };

pub struct GetUserByIdHandler {
    ctx: Arc<dyn PoolContext>,
    service: Arc<dyn UsersQueryService>
}

impl GetUserByIdHandler {
    pub fn new(
        ctx: Arc<dyn PoolContext>,
        service: Arc<dyn UsersQueryService>
    ) -> Self {
        Self { ctx, service }
    }
}

#[async_trait]
impl QueryHandler<GetUserByIdQuery> for GetUserByIdHandler {
    type Output = Option<GetUserByIdView>;
    
    async fn handle(&self, query: GetUserByIdQuery) -> Result<Self::Output, Box<dyn std::error::Error + Send + Sync>> {
        self.service.get_by_id(&*self.ctx, query.id)
            .await
            .map_err(|e| e.into())
    }
}
