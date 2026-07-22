use async_trait::{ async_trait };

use domain::user::{
    User,
    value_objects::{ UserId }
};

use crate::interfaces::query::{ QueryContext };

#[async_trait]
pub trait GetUserByIdQueryService: Send + Sync {
    async fn get_user_by_id(&self, context: &dyn QueryContext, id: UserId) -> Result<Option<User>, anyhow::Error>;
}
