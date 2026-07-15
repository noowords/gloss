use async_trait::{ async_trait };

use crate::application::common::persistence::{ TxContext };

use super::{
    User,
    value_objects::{ UserId }
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(
        &self,
        ctx: &mut dyn TxContext,
        user: &User
    ) -> Result<(), anyhow::Error>;
    
    async fn get_by_id(
        &self,
        ctx: &mut dyn TxContext,
        id: UserId
    ) -> Result<Option<User>, anyhow::Error>;
    
    async fn exists(
        &self,
        ctx: &mut dyn TxContext,
        id: UserId
    ) -> Result<bool, anyhow::Error>;
    
    async fn update(
        &self,
        ctx: &mut dyn TxContext,
        user: &User
    ) -> Result<(), anyhow::Error>;
    
    async fn remove(
        &self,
        ctx: &mut dyn TxContext,
        id: UserId
    ) -> Result<(), anyhow::Error>;
}
