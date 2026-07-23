use uuid::{ Uuid };
use sqlx::{ Type };

use domain::aggregates::user::value_objects::{ UserId };

#[derive(Clone, Type)]
#[sqlx(transparent)]
pub struct MySqlUserIdModel(Uuid);

impl MySqlUserIdModel {
    pub fn new(value: Uuid) -> Self {
        Self(value)
    }

    pub fn value(&self) -> Uuid {
        self.0.clone()
    }
}

impl TryFrom<MySqlUserIdModel> for UserId {
    type Error = anyhow::Error;

    fn try_from(model: MySqlUserIdModel) -> Result<Self, Self::Error> {
        Ok(Self::from(model.value()))
    }
}

impl From<UserId> for MySqlUserIdModel {
    fn from(id: UserId) -> Self {
        Self(id.uuid())
    }
}
