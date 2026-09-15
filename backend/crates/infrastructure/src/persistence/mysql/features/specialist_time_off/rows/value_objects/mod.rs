use chrono::{ NaiveDate, NaiveTime };
use uuid::Uuid;

crate::mysql_row_value!(MySqlSpecialistTimeOffIdRow, Uuid);
crate::mysql_row_value!(MySqlSpecialistTimeOffSpecialistIdRow, Uuid);
crate::mysql_row_value!(MySqlSpecialistTimeOffDateRow, NaiveDate);
crate::mysql_row_value!(MySqlSpecialistTimeOffTypeRow, String);
crate::mysql_row_value!(MySqlSpecialistTimeOffStartsAtRow, NaiveTime);
crate::mysql_row_value!(MySqlSpecialistTimeOffEndsAtRow, NaiveTime);
crate::mysql_row_value!(MySqlSpecialistTimeOffChargedLeaveMinutesRow, u16);
crate::mysql_row_value!(MySqlSpecialistTimeOffReasonRow, String);
