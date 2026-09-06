use crate::aggregates::{
    user::value_objects::{ UserId },
    service::{ Service }
};

use super::value_objects::{ AppointmentId, AppointmentDate, AppointmentTime, AppointmentDuration, AppointmentStatus };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Appointment {
    id: AppointmentId,
    specialist_id: UserId,
    client_id: UserId,
    date: AppointmentDate,
    time: AppointmentTime,
    duration: AppointmentDuration,
    status: AppointmentStatus
}

impl Appointment {
    pub fn create(
        specialist_id: UserId,
        client_id: UserId,
        date: AppointmentDate,
        time: AppointmentTime,
        services: &[Service]
    ) -> Result<Self, anyhow::Error> {
        if services.is_empty() {
            anyhow::bail!("Cannot create an appointment without any services");
        }
        
        Ok(Self {
            id: AppointmentId::generate(),
            specialist_id,
            client_id,
            date,
            time,
            duration: services
                .iter()
                .map(|s| u32::from(s.duration()))
                .sum::<u32>()
                .into(),
            status: AppointmentStatus::Pending
        })
    }
    
    pub fn restore(
        id: AppointmentId,
        specialist_id: UserId,
        client_id: UserId,
        date: AppointmentDate,
        time: AppointmentTime,
        duration: AppointmentDuration,
        status: AppointmentStatus
    ) -> Self {
        Self { id, specialist_id, client_id, date, time, duration, status }
    }

    pub fn id(&self) -> AppointmentId {
        self.id
    }

    pub fn specialist_id(&self) -> UserId {
        self.specialist_id
    }

    pub fn client_id(&self) -> UserId {
        self.client_id
    }

    pub fn time(&self) -> AppointmentTime {
        self.time
    }
    
    pub fn date(&self) -> AppointmentDate {
        self.date
    }
    
    pub fn duration(&self) -> AppointmentDuration {
        self.duration
    }

    pub fn status(&self) -> AppointmentStatus {
        self.status
    }
}
