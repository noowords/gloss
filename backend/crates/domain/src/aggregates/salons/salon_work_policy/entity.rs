use crate::aggregates::salons::salon::value_objects::{ SalonId };

use super::value_objects::{ SalonWorkPolicyWeeklyWorkMinutes, SalonWorkPolicyBookingStepMinutes, SalonWorkPolicyBookingHorizonDays, SalonWorkPolicyDefaultAnnualLeaveMinutes };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalonWorkPolicy {
    salon_id: SalonId,
    weekly_work_minutes: SalonWorkPolicyWeeklyWorkMinutes,
    booking_step_minutes: SalonWorkPolicyBookingStepMinutes,
    booking_horizon_days: SalonWorkPolicyBookingHorizonDays,
    default_annual_leave_minutes: SalonWorkPolicyDefaultAnnualLeaveMinutes
}

impl SalonWorkPolicy {
    pub fn create(
        salon_id: SalonId,
        weekly_work_minutes: SalonWorkPolicyWeeklyWorkMinutes
    ) -> Result<Self, anyhow::Error> {
        let booking_step_minutes = SalonWorkPolicyBookingStepMinutes::try_from(30u16)?;
        let booking_horizon_days = SalonWorkPolicyBookingHorizonDays::try_from(60u16)?;
        let default_annual_leave_minutes = SalonWorkPolicyDefaultAnnualLeaveMinutes::try_from(0u32)?;
        Self::restore(
            salon_id,
            weekly_work_minutes,
            booking_step_minutes,
            booking_horizon_days,
            default_annual_leave_minutes
        )
    }

    pub fn restore(
        salon_id: SalonId,
        weekly_work_minutes: SalonWorkPolicyWeeklyWorkMinutes,
        booking_step_minutes: SalonWorkPolicyBookingStepMinutes,
        booking_horizon_days: SalonWorkPolicyBookingHorizonDays,
        default_annual_leave_minutes: SalonWorkPolicyDefaultAnnualLeaveMinutes
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            salon_id,
            weekly_work_minutes,
            booking_step_minutes,
            booking_horizon_days,
            default_annual_leave_minutes
        })
    }

    pub fn salon_id(&self) -> SalonId {
        self.salon_id
    }

    pub fn weekly_work_minutes(&self) -> SalonWorkPolicyWeeklyWorkMinutes {
        self.weekly_work_minutes
    }

    pub fn booking_step_minutes(&self) -> SalonWorkPolicyBookingStepMinutes {
        self.booking_step_minutes
    }

    pub fn booking_horizon_days(&self) -> SalonWorkPolicyBookingHorizonDays {
        self.booking_horizon_days
    }

    pub fn default_annual_leave_minutes(&self) -> SalonWorkPolicyDefaultAnnualLeaveMinutes {
        self.default_annual_leave_minutes
    }
}
