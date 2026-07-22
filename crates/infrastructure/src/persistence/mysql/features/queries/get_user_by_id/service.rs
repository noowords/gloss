use async_trait::{ async_trait };

use domain::user::{
    User,
    value_objects::{ UserId }
};

use application::contexts::{ PoolContext };
use application::features::queries::get_user_by_id::{ GetUserByIdQueryService };

use crate::persistence::mysql::{
    contexts::{ MySqlPoolContext },
    models::user::{
        MySqlUserModel,
        value_objects::{ MySqlUserIdModel }
    }
};

#[derive(Default)]
pub struct MySqlGetUserByIdQueryService;

#[async_trait]
impl GetUserByIdQueryService for MySqlGetUserByIdQueryService {
    async fn get_user_by_id(&self, ctx: &dyn PoolContext, id: UserId) -> Result<Option<User>, anyhow::Error> {
        let ctx = ctx
            .downcast_ref::<MySqlPoolContext>()
            .ok_or_else(|| anyhow::anyhow!("Expected MySqlPoolContext inside MySqlGetUserByIdQueryRepository"))?;

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
            .fetch_optional(&ctx.pool)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        model.map(User::try_from).transpose()
    }
}
