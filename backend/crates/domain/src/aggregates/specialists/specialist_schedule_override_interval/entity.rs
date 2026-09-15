use crate::aggregates::specialists::specialist_schedule_override::value_objects::{ SpecialistScheduleOverrideId };

use super::value_objects::{ SpecialistScheduleOverrideIntervalId, SpecialistScheduleOverrideIntervalStartsAt, SpecialistScheduleOverrideIntervalEndsAt };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialistScheduleOverrideInterval {
    id: SpecialistScheduleOverrideIntervalId,
    override_id: SpecialistScheduleOverrideId,
    starts_at: SpecialistScheduleOverrideIntervalStartsAt,
    ends_at: SpecialistScheduleOverrideIntervalEndsAt
}

impl SpecialistScheduleOverrideInterval {
    pub fn create(
        override_id: SpecialistScheduleOverrideId,
        starts_at: SpecialistScheduleOverrideIntervalStartsAt,
        ends_at: SpecialistScheduleOverrideIntervalEndsAt
    ) -> Result<Self, anyhow::Error> {
        let id = SpecialistScheduleOverrideIntervalId::generate();
        Self::restore(
            id,
            override_id,
            starts_at,
            ends_at
        )
    }

    pub fn restore(
        id: SpecialistScheduleOverrideIntervalId,
        override_id: SpecialistScheduleOverrideId,
        starts_at: SpecialistScheduleOverrideIntervalStartsAt,
        ends_at: SpecialistScheduleOverrideIntervalEndsAt
    ) -> Result<Self, anyhow::Error> {
        if chrono::NaiveTime::from(starts_at) >= chrono::NaiveTime::from(ends_at) {
            anyhow::bail!("The interval must end after it starts");
        }

        Ok(Self {
            id,
            override_id,
            starts_at,
            ends_at
        })
    }

    pub fn id(&self) -> SpecialistScheduleOverrideIntervalId {
        self.id
    }

    pub fn override_id(&self) -> SpecialistScheduleOverrideId {
        self.override_id
    }

    pub fn starts_at(&self) -> SpecialistScheduleOverrideIntervalStartsAt {
        self.starts_at
    }

    pub fn ends_at(&self) -> SpecialistScheduleOverrideIntervalEndsAt {
        self.ends_at
    }
}
