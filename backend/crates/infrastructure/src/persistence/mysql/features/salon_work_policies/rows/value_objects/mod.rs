use uuid::Uuid;

crate::mysql_row_value!(MySqlSalonWorkPolicySalonIdRow, Uuid);
crate::mysql_row_value!(MySqlSalonWorkPolicyWeeklyWorkMinutesRow, u16);
crate::mysql_row_value!(MySqlSalonWorkPolicyBookingStepMinutesRow, u16);
crate::mysql_row_value!(MySqlSalonWorkPolicyBookingHorizonDaysRow, u16);
crate::mysql_row_value!(MySqlSalonWorkPolicyDefaultAnnualLeaveMinutesRow, u32);
