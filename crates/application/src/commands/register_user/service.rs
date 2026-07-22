use async_trait::{ async_trait };

use domain::user::{
    User,
    profile::{ Profile }
};

use crate::contexts::{ TxContext };

#[async_trait]
pub trait RegisterUserCommandService: Send + Sync {
    async fn save_user(&self, ctx: &mut dyn TxContext, user: &User) -> Result<(), anyhow::Error>;
    
    async fn save_profile(&self, ctx: &mut dyn TxContext, profile: &Profile) -> Result<(), anyhow::Error>;
}
