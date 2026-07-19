use std::cell::{ UnsafeCell };
use async_trait::{ async_trait };
use sqlx::{ Transaction, MySql };

use domain::{
    user::value_objects::{ UserId },
    profile::{ Profile }
};
use application::persistence::repositories::{ ProfileRepository };

use super::super::models::{ MySqlProfileRow };

pub struct MySqlProfileRepository<'a> {
    tx: UnsafeCell<&'a mut Transaction<'static, MySql>>
}

unsafe impl<'a> Send for MySqlProfileRepository<'a> {}
unsafe impl<'a> Sync for MySqlProfileRepository<'a> {}

impl<'a> MySqlProfileRepository<'a> {
    pub fn new(tx: &'a mut Transaction<'static, MySql>) -> Self {
        Self { tx: UnsafeCell::new(tx) }
    }

    #[inline(always)]
    fn tx_mut(&self) -> &mut sqlx::MySqlConnection {
        unsafe {
            let tx_ref = &mut *self.tx.get();
            tx_ref.as_mut()
        }
    }
}

#[async_trait]
impl<'a> ProfileRepository for MySqlProfileRepository<'a> {
    async fn create(&self, profile: &Profile) -> Result<(), anyhow::Error> {
        sqlx::query(
            r#"
            INSERT INTO profiles (user_id, first_name, last_name, avatar_url, bio)
            VALUES (?, ?, ?, ?, ?)
            "#
        )
            .bind(profile.user_id().value())
            .bind(profile.first_name())
            .bind(profile.last_name())
            .bind(profile.avatar_url())
            .bind(profile.bio())
            .execute(self.tx_mut())
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }
    
    async fn get_by_user_id(&self, user_id: UserId) -> Result<Option<Profile>, anyhow::Error> {
        let row: Option<MySqlProfileRow> = sqlx::query_as(
            r#"
            SELECT user_id, first_name, last_name, avatar_url, bio
            FROM profiles
            WHERE user_id = ?
            "#
        )
            .bind(user_id.value())
            .fetch_optional(self.tx_mut())
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        match row {
            Some(row) => Ok(Some(Profile::try_from(row)?)),
            None => Ok(None),
        }
    }
    
    async fn exists(&self, user_id: UserId) -> Result<bool, anyhow::Error> {
        let row = sqlx::query(
            r#"
            SELECT 1
            FROM profiles
            WHERE user_id = ?
            LIMIT 1
            "#
        )
            .bind(user_id.value())
            .fetch_optional(self.tx_mut())
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(row.is_some())
    }
    
    async fn update(&self, profile: &Profile) -> Result<(), anyhow::Error> {
        let row = MySqlProfileRow::from(profile);

        sqlx::query(
            r#"
            UPDATE profiles
            SET first_name = ?, last_name = ?, avatar_url = ?, bio = ?
            WHERE user_id = ?
            "#
        )
            .bind(row.first_name())
            .bind(row.last_name())
            .bind(row.avatar_url())
            .bind(row.bio())
            .bind(row.user_id())
            .execute(self.tx_mut())
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }
    
    async fn remove(&self, user_id: UserId) -> Result<(), anyhow::Error> {
        sqlx::query(
            r#"
            DELETE FROM profiles
            WHERE user_id = ?
            "#
        )
            .bind(user_id.value())
            .execute(self.tx_mut())
            .await
            .map_err(|e| anyhow::anyhow!(e.to_string()))?;

        Ok(())
    }
}
