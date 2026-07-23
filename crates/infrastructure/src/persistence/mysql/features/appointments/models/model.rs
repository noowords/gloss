use chrono::{ NaiveDate, NaiveTime };
use sqlx::{ FromRow };

use domain::aggregates::appointment::{ Appointment };

use crate::persistence::mysql::features::users::models::value_objects::{ MySqlUserIdModel };

use super::value_objects::{ MySqlAppointmentIdModel, MySqlAppointmentStatusModel };

#[derive(FromRow)]
pub struct MySqlAppointmentModel {
    id: MySqlAppointmentIdModel,
    master_id: MySqlUserIdModel,
    client_id: MySqlUserIdModel,
    date: NaiveDate,
    time: NaiveTime,
    status: Option<MySqlAppointmentStatusModel>
}

impl MySqlAppointmentModel {
    pub fn new(
        id: MySqlAppointmentIdModel,
        master_id: MySqlUserIdModel,
        client_id: MySqlUserIdModel,
        date: NaiveDate,
        time: NaiveTime,
        status: Option<MySqlAppointmentStatusModel>
    ) -> Self {
        Self {
            id,
            master_id,
            client_id,
            date,
            time,
            status
        }
    }

    pub fn id(&self) -> MySqlAppointmentIdModel {
        self.id.clone()
    }

    pub fn master_id(&self) -> MySqlUserIdModel {
        self.master_id.clone()
    }

    pub fn client_id(&self) -> MySqlUserIdModel {
        self.client_id.clone()
    }

    pub fn date(&self) -> NaiveDate {
        self.date
    }

    pub fn time(&self) -> NaiveTime {
        self.time
    }

    pub fn status(&self) -> Option<MySqlAppointmentStatusModel> {
        self.status.clone()
    }
}

impl TryFrom<MySqlAppointmentModel> for Appointment {
    type Error = anyhow::Error;

    fn try_from(model: MySqlAppointmentModel) -> Result<Self, Self::Error> {
        Ok(Self::restore(
            model.id.try_into()?,
            model.master_id.try_into()?,
            model.client_id.try_into()?,
            model.date,
            model.time,
            model.status
                .ok_or_else(|| anyhow::anyhow!("Missing appointment status in database"))?
                .try_into()?,
        ))
    }
}

impl From<&Appointment> for MySqlAppointmentModel {
    fn from(appointment: &Appointment) -> Self {
        Self {
            id: appointment.id().into(),
            master_id: appointment.master_id().into(),
            client_id: appointment.client_id().into(),
            date: appointment.date(),
            time: appointment.time(),
            status: Some(appointment.status().into())
        }
    }
}
