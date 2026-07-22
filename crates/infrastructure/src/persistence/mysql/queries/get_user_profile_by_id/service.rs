use async_trait::{ async_trait };

use domain::user::{
    profile::{ Profile },
    value_objects::{ UserId }
};

use application::contexts::{ PoolContext };
use application::queries::get_user_profile_by_id::{ GetUserProfileByIdQueryService };

use crate::persistence::mysql::{
    contexts::{ MySqlPoolContext },
    models::{
        user::value_objects::{ MySqlUserIdModel },
        profile::{ MySqlProfileModel }
    }
};

#[derive(Default)]
pub struct MySqlGetUserProfileByIdQueryService;

#[async_trait]
impl GetUserProfileByIdQueryService for MySqlGetUserProfileByIdQueryService {
    async fn get_user_profile_by_id(&self, ctx: &dyn PoolContext, id: UserId) -> Result<Option<Profile>, anyhow::Error> {
        let ctx = ctx
            .downcast_ref::<MySqlPoolContext>()
            .ok_or_else(|| anyhow::anyhow!("Expected MySqlPoolContext inside MySqlGetUserByIdQueryRepository"))?;

        let id_model: MySqlUserIdModel = id.into();

        let model: Option<MySqlProfileModel> = sqlx::query_as(
            r#"
            SELECT
                p.user_id,
                p.first_name,
                p.last_name,
                p.avatar_url,
                p.bio
            FROM profiles p
            WHERE p.user_id = ?
            LIMIT 1
            "#
        )
            .bind(id_model)
            .fetch_optional(&ctx.pool)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        model.map(Profile::try_from).transpose()
    }
}
