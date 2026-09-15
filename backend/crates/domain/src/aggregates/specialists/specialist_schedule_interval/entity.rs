use crate::aggregates::specialists::specialist_schedule::value_objects::{ SpecialistScheduleId };

use super::value_objects::{ SpecialistScheduleIntervalId, SpecialistScheduleIntervalWeekday, SpecialistScheduleIntervalStartsAt, SpecialistScheduleIntervalEndsAt };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleInterval {
    id: SpecialistScheduleIntervalId,
    schedule_id: SpecialistScheduleId,
    weekday: SpecialistScheduleIntervalWeekday,
    starts_at: SpecialistScheduleIntervalStartsAt,
    ends_at: SpecialistScheduleIntervalEndsAt
}

impl SpecialistScheduleInterval {
    pub fn create(
        schedule_id: SpecialistScheduleId,
        weekday: SpecialistScheduleIntervalWeekday,
        starts_at: SpecialistScheduleIntervalStartsAt,
        ends_at: SpecialistScheduleIntervalEndsAt
    ) -> Result<Self, anyhow::Error> {
        let id = SpecialistScheduleIntervalId::generate();
        Self::restore(
            id,
            schedule_id,
            weekday,
            starts_at,
            ends_at
        )
    }

    pub fn restore(
        id: SpecialistScheduleIntervalId,
        schedule_id: SpecialistScheduleId,
        weekday: SpecialistScheduleIntervalWeekday,
        starts_at: SpecialistScheduleIntervalStartsAt,
        ends_at: SpecialistScheduleIntervalEndsAt
    ) -> Result<Self, anyhow::Error> {
        if chrono::NaiveTime::from(starts_at) >= chrono::NaiveTime::from(ends_at) {
            anyhow::bail!("The interval must end after it starts");
        }

        Ok(Self {
            id,
            schedule_id,
            weekday,
            starts_at,
            ends_at
        })
    }

    pub fn id(&self) -> SpecialistScheduleIntervalId {
        self.id
    }

    pub fn schedule_id(&self) -> SpecialistScheduleId {
        self.schedule_id
    }

    pub fn weekday(&self) -> SpecialistScheduleIntervalWeekday {
        self.weekday
    }

    pub fn starts_at(&self) -> SpecialistScheduleIntervalStartsAt {
        self.starts_at
    }

    pub fn ends_at(&self) -> SpecialistScheduleIntervalEndsAt {
        self.ends_at
    }
}
