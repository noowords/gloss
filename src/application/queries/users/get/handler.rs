use std::sync::{ Arc };
use async_trait::{ async_trait };

use crate::domain::common::{ PoolContext };

use super::super::super::super::common::{ QueryHandler };

use super::super::{ UsersQueryService };

use super::{ GetUsersQuery, GetUsersView };

pub struct GetUsersHandler {
    ctx: Arc<dyn PoolContext>,
    service: Arc<dyn UsersQueryService>
}

impl GetUsersHandler {
    pub fn new(
        ctx: Arc<dyn PoolContext>,
        service: Arc<dyn UsersQueryService>
    ) -> Self {
        Self { ctx, service }
    }
}

#[async_trait]
impl QueryHandler<GetUsersQuery> for GetUsersHandler {
    type Output = GetUsersView;
    
    async fn handle(&self, query: GetUsersQuery) -> Result<Self::Output, Box<dyn std::error::Error + Send + Sync>> {
        self.service.get(&*self.ctx)
            .await
            .map_err(|e| e.into())
    }
}
