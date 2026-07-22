use async_trait::{ async_trait };

use domain::user::{
    User,
    value_objects::{ UserId }
};

use crate::contexts::{ PoolContext };

#[async_trait]
pub trait GetUserByIdQueryService: Send + Sync {
    async fn get_user_by_id(&self, ctx: &dyn PoolContext, id: UserId) -> Result<Option<User>, anyhow::Error>;
}
