use async_trait::{ async_trait };

use super::super::super::shared::{ TxContext };

use super::super::user::value_objects::{ UserId };

use super::{ Master };

#[async_trait]
pub trait MasterRepository: Send + Sync {
    async fn create(
        &self,
        uow: &mut dyn TxContext,
        master: &Master
    ) -> Result<(), anyhow::Error>;
    
    async fn get_by_user_id(
        &self,
        uow: &mut dyn TxContext,
        user_id: UserId
    ) -> Result<Option<Master>, anyhow::Error>;
    
    async fn exists(
        &self,
        uow: &mut dyn TxContext,
        user_id: UserId
    ) -> Result<bool, anyhow::Error>;
    
    async fn update(
        &self,
        uow: &mut dyn TxContext,
        master: &Master
    ) -> Result<(), anyhow::Error>;
    
    async fn remove(
        &self,
        uow: &mut dyn TxContext,
        user_id: UserId
    ) -> Result<(), anyhow::Error>;
}
