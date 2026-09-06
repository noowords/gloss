use async_trait::{ async_trait };

use crate::contracts::cqrs::query::{ QueryContext };

use super::dtos::{ User };

#[async_trait]
pub trait GetUsersQueryService: Send + Sync {
    async fn get_users(&self, context: &dyn QueryContext) -> Result<Vec<User>, anyhow::Error>;
}
