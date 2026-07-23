use async_trait::{ async_trait };

use domain::aggregates::user::{ User };

use crate::interfaces::query::{ QueryContext };

#[async_trait]
pub trait GetUsersQueryService: Send + Sync {
    async fn get_users(&self, context: &dyn QueryContext) -> Result<Vec<User>, anyhow::Error>;
}
