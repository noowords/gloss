use async_trait::{ async_trait };

use super::super::super::shared::{ TxContext };

use super::super::user::value_objects::{ UserId };

use super::{ Profile };

#[async_trait]
pub trait ProfileRepository: Send + Sync {
    async fn create(
        &self,
        uow: &mut dyn TxContext,
        profile: &Profile
    ) -> Result<(), anyhow::Error>;
    
    async fn get_by_user_id(
        &self,
        uow: &mut dyn TxContext,
        user_id: UserId
    ) -> Result<Option<Profile>, anyhow::Error>;
    
    async fn exists(
        &self,
        uow: &mut dyn TxContext,
        user_id: UserId
    ) -> Result<bool, anyhow::Error>;
    
    async fn update(
        &self,
        uow: &mut dyn TxContext,
        profile: &Profile
    ) -> Result<(), anyhow::Error>;
    
    async fn remove(
        &self,
        uow: &mut dyn TxContext,
        user_id: UserId
    ) -> Result<(), anyhow::Error>;
}
