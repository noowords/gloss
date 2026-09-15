mod weekly_work_minutes;
mod booking_step_minutes;
mod booking_horizon_days;
mod default_annual_leave_minutes;

pub use weekly_work_minutes::{ SalonWorkPolicyWeeklyWorkMinutes };
pub use booking_step_minutes::{ SalonWorkPolicyBookingStepMinutes };
pub use booking_horizon_days::{ SalonWorkPolicyBookingHorizonDays };
pub use default_annual_leave_minutes::{ SalonWorkPolicyDefaultAnnualLeaveMinutes };
