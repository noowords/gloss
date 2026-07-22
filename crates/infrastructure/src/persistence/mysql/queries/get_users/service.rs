use async_trait::{ async_trait };

use domain::user::{ User };

use application::contexts::{ PoolContext };
use application::queries::get_users::{ GetUsersQueryService };

use crate::persistence::mysql::{
    contexts::{ MySqlPoolContext },
    models::user::{ MySqlUserModel }
};

#[derive(Default)]
pub struct MySqlGetUsersQueryService;

#[async_trait]
impl GetUsersQueryService for MySqlGetUsersQueryService {
    async fn get_users(&self, ctx: &dyn PoolContext) -> Result<Vec<User>, anyhow::Error> {
        let ctx = ctx
            .downcast_ref::<MySqlPoolContext>()
            .ok_or_else(|| anyhow::anyhow!("Expected MySqlPoolContext inside MySqlGetUsersQueryRepository"))?;

        let model: Vec<MySqlUserModel> = sqlx::query_as(
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
            .fetch_all(&ctx.pool)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        model.into_iter().map(User::try_from).collect()
    }
}
