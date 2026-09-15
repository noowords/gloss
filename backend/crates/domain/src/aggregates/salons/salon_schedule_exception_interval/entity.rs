use crate::aggregates::salons::salon_schedule_exception::value_objects::{ SalonScheduleExceptionId };

use super::value_objects::{ SalonScheduleExceptionIntervalId, SalonScheduleExceptionIntervalStartsAt, SalonScheduleExceptionIntervalEndsAt };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalonScheduleExceptionInterval {
    id: SalonScheduleExceptionIntervalId,
    exception_id: SalonScheduleExceptionId,
    starts_at: SalonScheduleExceptionIntervalStartsAt,
    ends_at: SalonScheduleExceptionIntervalEndsAt
}

impl SalonScheduleExceptionInterval {
    pub fn create(
        exception_id: SalonScheduleExceptionId,
        starts_at: SalonScheduleExceptionIntervalStartsAt,
        ends_at: SalonScheduleExceptionIntervalEndsAt
    ) -> Result<Self, anyhow::Error> {
        let id = SalonScheduleExceptionIntervalId::generate();
        Self::restore(
            id,
            exception_id,
            starts_at,
            ends_at
        )
    }

    pub fn restore(
        id: SalonScheduleExceptionIntervalId,
        exception_id: SalonScheduleExceptionId,
        starts_at: SalonScheduleExceptionIntervalStartsAt,
        ends_at: SalonScheduleExceptionIntervalEndsAt
    ) -> Result<Self, anyhow::Error> {
        if chrono::NaiveTime::from(starts_at) >= chrono::NaiveTime::from(ends_at) {
            anyhow::bail!("The interval must end after it starts");
        }

        Ok(Self {
            id,
            exception_id,
            starts_at,
            ends_at
        })
    }

    pub fn id(&self) -> SalonScheduleExceptionIntervalId {
        self.id
    }

    pub fn exception_id(&self) -> SalonScheduleExceptionId {
        self.exception_id
    }

    pub fn starts_at(&self) -> SalonScheduleExceptionIntervalStartsAt {
        self.starts_at
    }

    pub fn ends_at(&self) -> SalonScheduleExceptionIntervalEndsAt {
        self.ends_at
    }
}
