use sqlx::{ FromRow };

use domain::aggregates::user::{
    profile::{ Profile },
    value_objects::{ UserId }
};

use super::super::user::value_objects::{ MySqlUserIdModel };

#[derive(Clone, FromRow)]
pub struct MySqlProfileModel {
    user_id: MySqlUserIdModel,
    first_name: String,
    last_name: Option<String>,
    avatar_url: Option<String>,
    bio: Option<String>
}

impl MySqlProfileModel {
    pub fn new(
        user_id: MySqlUserIdModel,
        first_name: String,
        last_name: Option<String>,
        avatar_url: Option<String>,
        bio: Option<String>
    ) -> Self {
        Self {
            user_id,
            first_name,
            last_name,
            avatar_url,
            bio
        }
    }

    pub fn user_id(&self) -> MySqlUserIdModel {
        self.user_id.clone()
    }

    pub fn first_name(&self) -> String {
        self.first_name.clone()
    }

    pub fn last_name(&self) -> Option<String> {
        self.last_name.clone()
    }

    pub fn avatar_url(&self) -> Option<String> {
        self.avatar_url.clone()
    }

    pub fn bio(&self) -> Option<String> {
        self.bio.clone()
    }
}

impl TryFrom<MySqlProfileModel> for Profile {
    type Error = anyhow::Error;

    fn try_from(model: MySqlProfileModel) -> Result<Self, Self::Error> {
        Ok(Self::restore(
            UserId::from(model.user_id.value()),
            model.first_name,
            model.last_name,
            model.avatar_url,
            model.bio
        ))
    }
}

impl From<&Profile> for MySqlProfileModel {
    fn from(profile: &Profile) -> Self {
        Self::new(
            profile.user_id().into(),
            profile.first_name().to_string(),
            profile.last_name(),
            profile.avatar_url(),
            profile.bio()
        )
    }
}
