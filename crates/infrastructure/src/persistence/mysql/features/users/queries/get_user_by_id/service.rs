use async_trait::{ async_trait };

use domain::aggregates::user::{
    User,
    value_objects::{ UserId }
};

use application::contracts::cqrs::query::{ QueryContext };
use application::features::users::queries::get_user_by_id::{ GetUserByIdQueryService };

use crate::persistence::mysql::{
    contracts::cqrs::query::{ MySqlQueryContext },
    features::users::models::{
        MySqlUserModel,
        value_objects::{ MySqlUserIdModel }
    },
};

#[derive(Default)]
pub struct MySqlGetUserByIdQueryService;

#[async_trait]
impl GetUserByIdQueryService for MySqlGetUserByIdQueryService {
    async fn get_user_by_id(&self, context: &dyn QueryContext, id: UserId) -> Result<Option<User>, anyhow::Error> {
        let pool = context.as_any()
            .downcast_ref::<MySqlQueryContext>()
            .map(|context| context.pool())
            .ok_or_else(|| anyhow::anyhow!("Invalid QueryContext"))?;

        let id_model: MySqlUserIdModel = id.into();

        let model: Option<MySqlUserModel> = sqlx::query_as(
            r#"
            SELECT
                u.id,
                u.role,
                u.phone,
                p.first_name,
                p.last_name,
                p.avatar_url,
                p.bio
            FROM users u
            JOIN profiles p ON u.id = p.user_id
            ORDER BY u.id DESC
            "#
        )
            .bind(id_model)
            .fetch_optional(pool)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        model.map(User::try_from).transpose()
    }
}
