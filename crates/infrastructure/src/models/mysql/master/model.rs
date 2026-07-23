use sqlx::{
    FromRow,
    types::{ Json }
};

use domain::aggregates::user::master::{ Master };

use super::super::user::value_objects::{ MySqlUserIdModel };

use super::value_objects::{ MySqlMasterScheduleModel };

#[derive(FromRow)]
pub struct MySqlMasterModel {
    user_id: MySqlUserIdModel,
    schedule: Json<MySqlMasterScheduleModel>
}

impl MySqlMasterModel {
    pub fn new(
        user_id: MySqlUserIdModel,
        schedule: Json<MySqlMasterScheduleModel>
    ) -> Self {
        Self {
            user_id,
            schedule
        }
    }

    pub fn user_id(&self) -> MySqlUserIdModel {
        self.user_id.clone()
    }

    pub fn schedule(&self) -> &Json<MySqlMasterScheduleModel> {
        &self.schedule
    }
}

impl TryFrom<MySqlMasterModel> for Master {
    type Error = anyhow::Error;

    fn try_from(model: MySqlMasterModel) -> Result<Self, Self::Error> {
        Ok(Self::restore(
            model.user_id().value().into(),
            model.schedule.0.try_into()?
        ))
    }
}

impl From<&Master> for MySqlMasterModel {
    fn from(master: &Master) -> Self {
        Self {
            user_id: master.user_id().into(),
            schedule: Json(master.schedule().into())
        }
    }
}
