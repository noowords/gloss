use async_trait::{ async_trait };
use uuid::{ Uuid };

use super::super::super::common::persistence::{ PoolContext };

use super::{
    get::{ GetUsersView },
    get_by_id::{ GetUserByIdView },
    get_profile_by_id::{ GetUserProfileByIdView }
};

#[async_trait]
pub trait UsersQueryService: Send + Sync {
    async fn get(
        &self,
        ctx: &dyn PoolContext
    ) -> Result<GetUsersView, anyhow::Error>;
    
    async fn get_by_id(
        &self,
        ctx: &dyn PoolContext,
        id: Uuid
    ) -> Result<Option<GetUserByIdView>, anyhow::Error>;
    
    async fn get_profile_by_id(
        &self,
        ctx: &dyn PoolContext,
        id: Uuid
    ) -> Result<Option<GetUserProfileByIdView>, anyhow::Error>;
}
