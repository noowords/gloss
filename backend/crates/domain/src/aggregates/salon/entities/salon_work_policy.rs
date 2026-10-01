use super::super::value_objects::{
    SalonId,
    SalonWorkPolicyWeeklyWorkMinutes,
    SalonWorkPolicyBookingStepMinutes,
    SalonWorkPolicyBookingHorizonDays,
    SalonWorkPolicyDefaultAnnualLeaveMinutes
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SalonWorkPolicy {
    salon_id: SalonId,
    weekly_work_minutes: SalonWorkPolicyWeeklyWorkMinutes,
    booking_step_minutes: SalonWorkPolicyBookingStepMinutes,
    booking_horizon_days: SalonWorkPolicyBookingHorizonDays,
    default_annual_leave_minutes: SalonWorkPolicyDefaultAnnualLeaveMinutes
}

// MARK: Constructors
impl SalonWorkPolicy {
    pub fn create(
        salon_id: SalonId,
        weekly_work_minutes: SalonWorkPolicyWeeklyWorkMinutes,
        booking_step_minutes: SalonWorkPolicyBookingStepMinutes,
        booking_horizon_days: SalonWorkPolicyBookingHorizonDays,
        default_annual_leave_minutes: SalonWorkPolicyDefaultAnnualLeaveMinutes
    ) -> Self {
        Self { salon_id, weekly_work_minutes, booking_step_minutes, booking_horizon_days, default_annual_leave_minutes }
    }

    pub fn restore(
        salon_id: SalonId,
        weekly_work_minutes: SalonWorkPolicyWeeklyWorkMinutes,
        booking_step_minutes: SalonWorkPolicyBookingStepMinutes,
        booking_horizon_days: SalonWorkPolicyBookingHorizonDays,
        default_annual_leave_minutes: SalonWorkPolicyDefaultAnnualLeaveMinutes
    ) -> Self {
        Self { salon_id, weekly_work_minutes, booking_step_minutes, booking_horizon_days, default_annual_leave_minutes }
    }
}

// MARK: Behavior
impl SalonWorkPolicy {
    pub fn change_weekly_work_minutes(
        &mut self,
        weekly_work_minutes: SalonWorkPolicyWeeklyWorkMinutes
    ) {
        self.weekly_work_minutes = weekly_work_minutes;
    }

    pub fn change_booking_step_minutes(
        &mut self,
        booking_step_minutes: SalonWorkPolicyBookingStepMinutes
    ) {
        self.booking_step_minutes = booking_step_minutes;
    }

    pub fn change_booking_horizon_days(
        &mut self,
        booking_horizon_days: SalonWorkPolicyBookingHorizonDays
    ) {
        self.booking_horizon_days = booking_horizon_days;
    }

    pub fn change_default_annual_leave_minutes(
        &mut self,
        default_annual_leave_minutes: SalonWorkPolicyDefaultAnnualLeaveMinutes
    ) {
        self.default_annual_leave_minutes = default_annual_leave_minutes;
    }
}

// MARK: Getters
impl SalonWorkPolicy {
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
