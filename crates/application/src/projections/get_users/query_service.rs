use async_trait::{ async_trait };

use domain::user::{ User };

use crate::contexts::{ PoolContext };

#[async_trait]
pub trait GetUsersQueryService: Send + Sync {
    async fn get_users(&self, ctx: &dyn PoolContext) -> Result<Vec<User>, anyhow::Error>;
}
