use async_trait::{ async_trait };

use super::super::super::common::{ TxContext };

use super::{
    User,
    value_objects::{ UserId }
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(
        &self,
        uow: &mut dyn TxContext,
        user: &User
    ) -> Result<(), anyhow::Error>;
    
    async fn get_by_id(
        &self,
        uow: &mut dyn TxContext,
        id: UserId
    ) -> Result<Option<User>, anyhow::Error>;
    
    async fn exists(
        &self,
        uow: &mut dyn TxContext,
        id: UserId
    ) -> Result<bool, anyhow::Error>;
    
    async fn update(
        &self,
        uow: &mut dyn TxContext,
        user: &User
    ) -> Result<(), anyhow::Error>;
    
    async fn remove(
        &self,
        uow: &mut dyn TxContext,
        id: UserId
    ) -> Result<(), anyhow::Error>;
}
