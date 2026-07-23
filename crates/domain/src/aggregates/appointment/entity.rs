use chrono::{ NaiveDate, NaiveTime };

use crate::aggregates::user::value_objects::{ UserId };
use super::value_objects::{ AppointmentId, AppointmentStatus };

pub struct Appointment {
    id: AppointmentId,
    master_id: UserId,
    client_id: UserId,
    date: NaiveDate,
    time: NaiveTime,
    status: AppointmentStatus
}

impl Appointment {
    pub fn schedule(
        master_id: UserId,
        client_id: UserId,
        date: NaiveDate,
        time: NaiveTime
    ) -> Self {
        Self {
            id: AppointmentId::generate(),
            master_id,
            client_id,
            date,
            time,
            status: AppointmentStatus::Pending
        }
    }
    
    pub fn restore(
        id: AppointmentId,
        master_id: UserId,
        client_id: UserId,
        date: NaiveDate,
        time: NaiveTime,
        status: AppointmentStatus
    ) -> Self {
        Self { id, master_id, client_id, date, time, status }
    }

    pub fn id(&self) -> AppointmentId {
        self.id
    }

    pub fn master_id(&self) -> UserId {
        self.master_id
    }

    pub fn client_id(&self) -> UserId {
        self.client_id
    }

    pub fn time(&self) -> NaiveTime {
        self.time
    }
    
    pub fn date(&self) -> NaiveDate {
        self.date
    }
    
    pub fn status(&self) -> AppointmentStatus {
        self.status.clone()
    }
}
