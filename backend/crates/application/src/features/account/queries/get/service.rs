use async_trait::{ async_trait };

use domain::aggregates::users::user::value_objects::{ UserId };

use crate::common::cqrs::query::{ QueryContext };

use super::dtos::{ User };

#[async_trait]
pub trait GetAccountQueryService: Send + Sync + 'static {
    async fn get_user_by_id(&self, context: &dyn QueryContext, id: UserId) -> Result<Option<User>, anyhow::Error>;
}
