use async_trait::{ async_trait };

use domain::aggregates::user::{
    profile::{ Profile },
    value_objects::{ UserId }
};

use application::contracts::cqrs::query::{ QueryContext };
use application::features::users::queries::get_user_profile_by_id::{ GetUserProfileByIdQueryService };

use crate::persistence::mysql::{
    contracts::cqrs::query::{ MySqlQueryContext },
    features::{
        users::models::value_objects::{ MySqlUserIdModel },
        profiles::models::{ MySqlProfileModel }
    },
};

#[derive(Default)]
pub struct MySqlGetUserProfileByIdQueryService;

#[async_trait]
impl GetUserProfileByIdQueryService for MySqlGetUserProfileByIdQueryService {
    async fn get_user_profile_by_id(&self, context: &dyn QueryContext, id: UserId) -> Result<Option<Profile>, anyhow::Error> {
        let pool = context.as_any()
            .downcast_ref::<MySqlQueryContext>()
            .map(|context| context.pool())
            .ok_or_else(|| anyhow::anyhow!("Invalid QueryContext"))?;

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
            .fetch_optional(pool)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        model.map(Profile::try_from).transpose()
    }
}
