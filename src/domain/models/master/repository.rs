use async_trait::{ async_trait };

use crate::application::common::persistence::{ TxContext };

use super::super::user::value_objects::{ UserId };

use super::{ Master };

#[async_trait]
pub trait MasterRepository: Send + Sync {
    async fn create(
        &self,
        ctx: &mut dyn TxContext,
        master: &Master
    ) -> Result<(), anyhow::Error>;
    
    async fn get_by_user_id(
        &self,
        ctx: &mut dyn TxContext,
        user_id: UserId
    ) -> Result<Option<Master>, anyhow::Error>;
    
    async fn exists(
        &self,
        ctx: &mut dyn TxContext,
        user_id: UserId
    ) -> Result<bool, anyhow::Error>;
    
    async fn update(
        &self,
        ctx: &mut dyn TxContext,
        master: &Master
    ) -> Result<(), anyhow::Error>;
    
    async fn remove(
        &self,
        ctx: &mut dyn TxContext,
        user_id: UserId
    ) -> Result<(), anyhow::Error>;
}
