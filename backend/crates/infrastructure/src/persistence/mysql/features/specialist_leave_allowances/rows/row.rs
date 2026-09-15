use domain::aggregates::specialists::specialist_leave_allowance::SpecialistLeaveAllowance;

use super::value_objects::{
    MySqlSpecialistLeaveAllowanceAllocatedMinutesRow,
    MySqlSpecialistLeaveAllowanceSpecialistIdRow,
    MySqlSpecialistLeaveAllowanceYearRow
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlSpecialistLeaveAllowanceRow {
    pub specialist_id: MySqlSpecialistLeaveAllowanceSpecialistIdRow,
    pub year: MySqlSpecialistLeaveAllowanceYearRow,
    pub allocated_minutes: MySqlSpecialistLeaveAllowanceAllocatedMinutesRow
}

impl TryFrom<MySqlSpecialistLeaveAllowanceRow> for SpecialistLeaveAllowance {
    type Error = anyhow::Error;

    fn try_from(row: MySqlSpecialistLeaveAllowanceRow) -> Result<Self, Self::Error> {
        SpecialistLeaveAllowance::restore(
            uuid::Uuid::from(row.specialist_id).into(),
            u16::from(row.year).try_into()?,
            u32::from(row.allocated_minutes).try_into()?
        )
    }
}

impl From<&SpecialistLeaveAllowance> for MySqlSpecialistLeaveAllowanceRow {
    fn from(entity: &SpecialistLeaveAllowance) -> Self {
        Self {
            specialist_id: uuid::Uuid::from(entity.specialist_id()).into(),
            year: u16::from(entity.year()).into(),
            allocated_minutes: u32::from(entity.allocated_minutes()).into()
        }
    }
}
