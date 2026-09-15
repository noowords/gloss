use crate::aggregates::specialists::specialist::value_objects::{ SpecialistId };

use super::value_objects::{ SpecialistLeaveAllowanceYear, SpecialistLeaveAllowanceAllocatedMinutes };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialistLeaveAllowance {
    specialist_id: SpecialistId,
    year: SpecialistLeaveAllowanceYear,
    allocated_minutes: SpecialistLeaveAllowanceAllocatedMinutes
}

impl SpecialistLeaveAllowance {
    pub fn create(
        specialist_id: SpecialistId,
        year: SpecialistLeaveAllowanceYear,
        allocated_minutes: SpecialistLeaveAllowanceAllocatedMinutes
    ) -> Result<Self, anyhow::Error> {
        Self::restore(
            specialist_id,
            year,
            allocated_minutes
        )
    }

    pub fn restore(
        specialist_id: SpecialistId,
        year: SpecialistLeaveAllowanceYear,
        allocated_minutes: SpecialistLeaveAllowanceAllocatedMinutes
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            specialist_id,
            year,
            allocated_minutes
        })
    }

    pub fn specialist_id(&self) -> SpecialistId {
        self.specialist_id
    }

    pub fn year(&self) -> SpecialistLeaveAllowanceYear {
        self.year
    }

    pub fn allocated_minutes(&self) -> SpecialistLeaveAllowanceAllocatedMinutes {
        self.allocated_minutes
    }
}
