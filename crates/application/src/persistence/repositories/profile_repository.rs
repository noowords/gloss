use async_trait::{ async_trait };

use domain::{
    user::value_objects::{ UserId },
    profile::{ Profile }
};

#[async_trait]
pub trait ProfileRepository: Send + Sync {
    async fn create(&self, profile: &Profile) -> Result<(), anyhow::Error>;
    
    async fn get_by_user_id(&self, user_id: UserId) -> Result<Option<Profile>, anyhow::Error>;
    
    async fn exists(&self, user_id: UserId) -> Result<bool, anyhow::Error>;
    
    async fn update(&self, profile: &Profile) -> Result<(), anyhow::Error>;
    
    async fn remove(&self, user_id: UserId) -> Result<(), anyhow::Error>;
}
