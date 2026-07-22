use chrono::{ NaiveTime };
use serde::{ Serialize, Deserialize };

use domain::user::master::value_objects::{ MasterWorkDay };

#[derive(Clone, Serialize, Deserialize)]
pub struct MySqlMasterWorkDayModel {
    slots: Vec<NaiveTime>
}

impl MySqlMasterWorkDayModel {
    pub fn new(slots: Vec<NaiveTime>) -> Self {
        Self { slots }
    }

    pub fn slots(&self) -> Vec<NaiveTime> {
        self.slots.clone()
    }
}

impl TryFrom<MySqlMasterWorkDayModel> for MasterWorkDay {
    type Error = anyhow::Error;
    
    fn try_from(model: MySqlMasterWorkDayModel) -> Result<Self, Self::Error> {
        Ok(Self::from_slots(model.slots))
    }
}

impl From<&MasterWorkDay> for MySqlMasterWorkDayModel {
    fn from(work_day: &MasterWorkDay) -> Self {
        Self { slots: work_day.slots().clone() }
    }
}
