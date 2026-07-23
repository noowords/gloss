use std::collections::{ HashMap };
use chrono::{ Weekday };
use serde::{ Serialize, Deserialize };

use domain::aggregates::user::master::value_objects::{ MasterSchedule };

use super::{ MySqlMasterWorkDayModel };

#[derive(Serialize, Deserialize)]
pub struct MySqlMasterScheduleModel {
    work_days: HashMap<Weekday, MySqlMasterWorkDayModel>,
    max_advance_booking_days: u32
}

impl MySqlMasterScheduleModel {
    pub fn new(
        work_days: HashMap<Weekday, MySqlMasterWorkDayModel>,
        max_advance_booking_days: u32
    ) -> Self {
        Self { work_days, max_advance_booking_days }
    }

    pub fn work_days(&self) -> HashMap<Weekday, MySqlMasterWorkDayModel> {
        self.work_days.clone()
    }

    pub fn max_advance_booking_days(&self) -> u32 {
        self.max_advance_booking_days
    }
}

impl TryFrom<MySqlMasterScheduleModel> for MasterSchedule {
    type Error = anyhow::Error;

    fn try_from(model: MySqlMasterScheduleModel) -> Result<Self, Self::Error> {
        Ok(Self::new(
            model.work_days()
                .iter()
                .map(|(weekday, workday)| Ok((*weekday, workday.clone().try_into()?)))
                .collect::<Result<HashMap<_, _>, anyhow::Error>>()?,
            model.max_advance_booking_days()
        ))
    }
}

impl From<&MasterSchedule> for MySqlMasterScheduleModel {
    fn from(schedule: &MasterSchedule) -> Self {
        Self {
            work_days: schedule.work_days()
                .iter()
                .map(|(weekday, workday)| (*weekday, workday.into()))
                .collect(),
            max_advance_booking_days: schedule.max_advance_booking_days()
        }
    }
}
