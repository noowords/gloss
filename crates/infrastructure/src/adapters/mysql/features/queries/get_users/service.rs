use async_trait::{ async_trait };

use domain::user::{ User };

use application::interfaces::query::{ QueryContext };
use application::features::queries::get_users::{ GetUsersQueryService };

use crate::adapters::mysql::interfaces::query::{ MySqlQueryContext };
use crate::models::mysql::user::{ MySqlUserModel };

#[derive(Default)]
pub struct MySqlGetUsersQueryService;

#[async_trait]
impl GetUsersQueryService for MySqlGetUsersQueryService {
    async fn get_users(&self, context: &dyn QueryContext) -> Result<Vec<User>, anyhow::Error> {
        let pool = context.as_any()
            .downcast_ref::<MySqlQueryContext>()
            .map(|context| context.pool())
            .ok_or_else(|| anyhow::anyhow!("Invalid QueryContext"))?;

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
            .fetch_all(pool)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        model.into_iter().map(User::try_from).collect()
    }
}
