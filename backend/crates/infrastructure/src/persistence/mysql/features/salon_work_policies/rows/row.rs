use domain::aggregates::salons::salon_work_policy::SalonWorkPolicy;

use super::value_objects::{
    MySqlSalonWorkPolicyBookingHorizonDaysRow,
    MySqlSalonWorkPolicyBookingStepMinutesRow,
    MySqlSalonWorkPolicyDefaultAnnualLeaveMinutesRow,
    MySqlSalonWorkPolicySalonIdRow,
    MySqlSalonWorkPolicyWeeklyWorkMinutesRow
};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct MySqlSalonWorkPolicyRow {
    pub salon_id: MySqlSalonWorkPolicySalonIdRow,
    pub weekly_work_minutes: MySqlSalonWorkPolicyWeeklyWorkMinutesRow,
    pub booking_step_minutes: MySqlSalonWorkPolicyBookingStepMinutesRow,
    pub booking_horizon_days: MySqlSalonWorkPolicyBookingHorizonDaysRow,
    pub default_annual_leave_minutes: MySqlSalonWorkPolicyDefaultAnnualLeaveMinutesRow
}

impl TryFrom<MySqlSalonWorkPolicyRow> for SalonWorkPolicy {
    type Error = anyhow::Error;

    fn try_from(row: MySqlSalonWorkPolicyRow) -> Result<Self, Self::Error> {
        SalonWorkPolicy::restore(
            uuid::Uuid::from(row.salon_id).into(),
            u16::from(row.weekly_work_minutes).try_into()?,
            u16::from(row.booking_step_minutes).try_into()?,
            u16::from(row.booking_horizon_days).try_into()?,
            u32::from(row.default_annual_leave_minutes).try_into()?
        )
    }
}

impl From<&SalonWorkPolicy> for MySqlSalonWorkPolicyRow {
    fn from(entity: &SalonWorkPolicy) -> Self {
        Self {
            salon_id: uuid::Uuid::from(entity.salon_id()).into(),
            weekly_work_minutes: u16::from(entity.weekly_work_minutes()).into(),
            booking_step_minutes: u16::from(entity.booking_step_minutes()).into(),
            booking_horizon_days: u16::from(entity.booking_horizon_days()).into(),
            default_annual_leave_minutes: u32::from(entity.default_annual_leave_minutes()).into()
        }
    }
}
