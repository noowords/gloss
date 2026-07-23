use async_trait::{ async_trait };

use domain::aggregates::user::{
    User,
    value_objects::{ UserId }
};

use crate::contracts::query::{ QueryContext };

#[async_trait]
pub trait GetUserByIdQueryService: Send + Sync {
    async fn get_user_by_id(&self, context: &dyn QueryContext, id: UserId) -> Result<Option<User>, anyhow::Error>;
}
