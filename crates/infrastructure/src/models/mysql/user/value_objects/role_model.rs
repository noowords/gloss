use sqlx::{ Type };

use domain::aggregates::user::value_objects::{ UserRole };

#[derive(Clone, Type)]
#[sqlx(transparent)]
pub struct MySqlUserRoleModel(String);

impl MySqlUserRoleModel {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> String {
        self.0.clone()
    }
}

impl TryFrom<MySqlUserRoleModel> for UserRole {
    type Error = anyhow::Error;

    fn try_from(model: MySqlUserRoleModel) -> Result<Self, Self::Error> {
        UserRole::try_from(model.value().as_str())
    }
}

impl From<UserRole> for MySqlUserRoleModel {
    fn from(role: UserRole) -> Self {
        Self::new(role.into())
    }
}
