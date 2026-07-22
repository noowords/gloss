use sqlx::{ FromRow };

use domain::user::{ User, profile::Profile };

use super::value_objects::{ MySqlUserIdModel, MySqlUserRoleModel, MySqlUserPhoneModel };

#[derive(FromRow)]
pub struct MySqlUserModel {
    id: MySqlUserIdModel,
    role: MySqlUserRoleModel,
    phone: Option<MySqlUserPhoneModel>,
    first_name: String,
    last_name: Option<String>,
    avatar_url: Option<String>,
    bio: Option<String>
}

impl MySqlUserModel {
    pub fn new(
        id: MySqlUserIdModel,
        role: MySqlUserRoleModel,
        phone: Option<MySqlUserPhoneModel>,
        first_name: String,
        last_name: Option<String>,
        avatar_url: Option<String>,
        bio: Option<String>
    ) -> Self {
        Self {
            id: id.clone(),
            role,
            phone,
            first_name,
            last_name,
            avatar_url,
            bio,
        }
    }

    pub fn id(&self) -> MySqlUserIdModel {
        self.id.clone()
    }
    
    pub fn role(&self) -> MySqlUserRoleModel {
        self.role.clone()
    }
    
    pub fn phone(&self) -> Option<MySqlUserPhoneModel> {
        self.phone.clone()
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

impl TryFrom<MySqlUserModel> for User {
    type Error = anyhow::Error;
    
    fn try_from(model: MySqlUserModel) -> Result<Self, Self::Error> {
        let MySqlUserModel {
            id,
            role,
            phone,
            first_name,
            last_name,
            avatar_url,
            bio
        } = model;

        Ok(Self::restore(
            id.clone().try_into()?,
            role.try_into()?,
            phone.map(|p| p.try_into()).transpose()?,
            Profile::restore(
                id.try_into()?,
                first_name,
                last_name,
                avatar_url,
                bio
            )
        ))
    }
}

impl From<&User> for MySqlUserModel {
    fn from(user: &User) -> Self {
        Self::new(
            user.id().into(),
            user.role().clone().into(),
            user.phone().as_ref().map(|p| p.clone().into()),
            user.profile().first_name().clone(),
            user.profile().last_name().clone(),
            user.profile().avatar_url().clone(),
            user.profile().bio().clone(),
        )
    }
}
