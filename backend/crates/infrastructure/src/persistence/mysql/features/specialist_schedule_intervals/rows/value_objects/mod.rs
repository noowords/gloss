use chrono::NaiveTime;
use uuid::Uuid;

crate::mysql_row_value!(MySqlSpecialistScheduleIntervalIdRow, Uuid);
crate::mysql_row_value!(MySqlSpecialistScheduleIntervalScheduleIdRow, Uuid);
crate::mysql_row_value!(MySqlSpecialistScheduleIntervalWeekdayRow, u8);
crate::mysql_row_value!(MySqlSpecialistScheduleIntervalStartsAtRow, NaiveTime);
crate::mysql_row_value!(MySqlSpecialistScheduleIntervalEndsAtRow, NaiveTime);
