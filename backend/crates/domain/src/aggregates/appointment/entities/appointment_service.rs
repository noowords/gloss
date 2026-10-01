use crate::aggregates::{
    appointment::value_objects::AppointmentId,
    service::value_objects::ServiceId
};

use super::super::value_objects::{
    AppointmentServiceRole,
    AppointmentServiceNameSnapshot,
    AppointmentServicePriceSnapshot,
    AppointmentServiceDurationMinutesSnapshot
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppointmentService {
    appointment_id: AppointmentId,
    service_id: ServiceId,
    role: AppointmentServiceRole,
    service_name_snapshot: AppointmentServiceNameSnapshot,
    price_snapshot: AppointmentServicePriceSnapshot,
    duration_minutes_snapshot: AppointmentServiceDurationMinutesSnapshot
}

// MARK: Constructors
impl AppointmentService {
    pub fn create(
        appointment_id: AppointmentId,
        service_id: ServiceId,
        role: AppointmentServiceRole,
        service_name_snapshot: AppointmentServiceNameSnapshot,
        price_snapshot: AppointmentServicePriceSnapshot,
        duration_minutes_snapshot: AppointmentServiceDurationMinutesSnapshot
    ) -> Self {
        Self { appointment_id, service_id, role, service_name_snapshot, price_snapshot, duration_minutes_snapshot }
    }

    pub fn restore(
        appointment_id: AppointmentId,
        service_id: ServiceId,
        role: AppointmentServiceRole,
        service_name_snapshot: AppointmentServiceNameSnapshot,
        price_snapshot: AppointmentServicePriceSnapshot,
        duration_minutes_snapshot: AppointmentServiceDurationMinutesSnapshot
    ) -> Self {
        Self { appointment_id, service_id, role, service_name_snapshot, price_snapshot, duration_minutes_snapshot }
    }
}

// MARK: Getters
impl AppointmentService {
    pub fn appointment_id(&self) -> AppointmentId {
        self.appointment_id
    }

    pub fn service_id(&self) -> ServiceId {
        self.service_id
    }

    pub fn role(&self) -> AppointmentServiceRole {
        self.role
    }

    pub fn service_name_snapshot(&self) -> &AppointmentServiceNameSnapshot {
        &self.service_name_snapshot
    }

    pub fn price_snapshot(&self) -> &AppointmentServicePriceSnapshot {
        &self.price_snapshot
    }

    pub fn duration_minutes_snapshot(&self) -> AppointmentServiceDurationMinutesSnapshot {
        self.duration_minutes_snapshot
    }
}
