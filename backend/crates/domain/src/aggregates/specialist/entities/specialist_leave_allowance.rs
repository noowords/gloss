use super::super::value_objects::{
    SpecialistId,
    SpecialistLeaveAllowanceYear,
    SpecialistLeaveAllowanceAllocatedMinutes
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialistLeaveAllowance {
    specialist_id: SpecialistId,
    year: SpecialistLeaveAllowanceYear,
    allocated_minutes: SpecialistLeaveAllowanceAllocatedMinutes
}

// MARK: Constructors
impl SpecialistLeaveAllowance {
    pub fn create(
        specialist_id: SpecialistId,
        year: SpecialistLeaveAllowanceYear,
        allocated_minutes: SpecialistLeaveAllowanceAllocatedMinutes
    ) -> Self {
        Self { specialist_id, year, allocated_minutes }
    }

    pub fn restore(
        specialist_id: SpecialistId,
        year: SpecialistLeaveAllowanceYear,
        allocated_minutes: SpecialistLeaveAllowanceAllocatedMinutes
    ) -> Self {
        Self { specialist_id, year, allocated_minutes }
    }
}

// MARK: Getters
impl SpecialistLeaveAllowance {
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
