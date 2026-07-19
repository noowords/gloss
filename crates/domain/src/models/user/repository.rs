use async_trait::{ async_trait };

use super::{
    User,
    value_objects::{ UserId }
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn create(&self, user: &User) -> Result<(), anyhow::Error>;

    async fn get_by_id(&self, id: UserId) -> Result<Option<User>, anyhow::Error>;

    async fn exists(&self, id: UserId) -> Result<bool, anyhow::Error>;

    async fn update(&self, user: &User) -> Result<(), anyhow::Error>;

    async fn remove(&self, id: UserId) -> Result<(), anyhow::Error>;
}
