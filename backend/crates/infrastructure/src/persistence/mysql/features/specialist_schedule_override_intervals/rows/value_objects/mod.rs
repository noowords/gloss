use chrono::NaiveTime;
use uuid::Uuid;

crate::mysql_row_value!(MySqlSpecialistScheduleOverrideIntervalIdRow, Uuid);
crate::mysql_row_value!(MySqlSpecialistScheduleOverrideIntervalOverrideIdRow, Uuid);
crate::mysql_row_value!(MySqlSpecialistScheduleOverrideIntervalStartsAtRow, NaiveTime);
crate::mysql_row_value!(MySqlSpecialistScheduleOverrideIntervalEndsAtRow, NaiveTime);
