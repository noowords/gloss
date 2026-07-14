use async_trait::{ async_trait };
use uuid::{ Uuid };

use crate::application::queries::{
    users::{
        UsersQueryService,
        get::{ GetUsersView },
        get_by_id::{ GetUserByIdView },
        get_profile_by_id::{ GetUserProfileByIdView }
    }
};
use crate::domain::shared::{ PoolContext };

use super::super::shared::{ MySqlPoolContext };

#[derive(Default)]
pub struct MySqlUsersQueryService;

impl MySqlUsersQueryService {
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl UsersQueryService for MySqlUsersQueryService {
    async fn get(
        &self,
        ctx: &dyn PoolContext
    ) -> Result<GetUsersView, anyhow::Error> {
        let ctx = ctx
            .downcast_ref::<MySqlPoolContext>()
            .ok_or_else(|| anyhow::anyhow!("Expected MySqlPoolContext inside MySqlGetUsersQueryRepository"))?;
            
        let users = sqlx::query_as(
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

        Ok(GetUsersView { users })
    }
    
    async fn get_by_id(
        &self,
        ctx: &dyn PoolContext,
        id: Uuid
    ) -> Result<Option<GetUserByIdView>, anyhow::Error> {
        let ctx = ctx
            .downcast_ref::<MySqlPoolContext>()
            .ok_or_else(|| anyhow::anyhow!("Expected MySqlPoolContext inside MySqlGetUserByIdQueryRepository"))?;
        
        sqlx::query_as(
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
            WHERE u.id = ?
            LIMIT 1
            "#
        )
            .bind(id)
            .fetch_optional(&ctx.pool)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))
    }
    
    async fn get_profile_by_id(
        &self,
        ctx: &dyn PoolContext,
        id: Uuid
    ) -> Result<Option<GetUserProfileByIdView>, anyhow::Error> {
        let ctx = ctx
            .downcast_ref::<MySqlPoolContext>()
            .ok_or_else(|| anyhow::anyhow!("Expected MySqlPoolContext inside MySqlGetUserByIdQueryRepository"))?;
        
        sqlx::query_as(
            r#"
            SELECT 
                u.id,
                u.role,
                u.phone,
                p.first_name,
                p.last_name,
                p.avatar_url,
                p.bio
            FROM profiles p
            JOIN users u ON p.user_id = u.id
            WHERE p.user_id = ?
            LIMIT 1
            "#
        )
            .bind(id)
            .fetch_optional(&ctx.pool)
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))
    }
}
