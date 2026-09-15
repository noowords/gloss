use chrono::NaiveDate;
use uuid::Uuid;

crate::mysql_row_value!(MySqlSpecialistScheduleOverrideIdRow, Uuid);
crate::mysql_row_value!(MySqlSpecialistScheduleOverrideSpecialistIdRow, Uuid);
crate::mysql_row_value!(MySqlSpecialistScheduleOverrideDateRow, NaiveDate);
crate::mysql_row_value!(MySqlSpecialistScheduleOverrideReasonRow, String);
