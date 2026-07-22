use sqlx::{ Type };

use domain::user::value_objects::{ UserPhone };

#[derive(Clone, Type)]
#[sqlx(transparent)]
pub struct MySqlUserPhoneModel(String);

impl MySqlUserPhoneModel {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> String {
        self.0.clone()
    }
}

impl TryFrom<MySqlUserPhoneModel> for UserPhone {
    type Error = anyhow::Error;
    
    fn try_from(model: MySqlUserPhoneModel) -> Result<Self, Self::Error> {
        UserPhone::try_from(model.value().as_str())
    }
}

impl From<UserPhone> for MySqlUserPhoneModel {
    fn from(phone: UserPhone) -> Self {
        Self::new(phone.into())
    }
}
