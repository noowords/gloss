use chrono::NaiveTime;
use uuid::Uuid;

crate::mysql_row_value!(MySqlSalonScheduleIntervalIdRow, Uuid);
crate::mysql_row_value!(MySqlSalonScheduleIntervalSalonIdRow, Uuid);
crate::mysql_row_value!(MySqlSalonScheduleIntervalWeekdayRow, u8);
crate::mysql_row_value!(MySqlSalonScheduleIntervalStartsAtRow, NaiveTime);
crate::mysql_row_value!(MySqlSalonScheduleIntervalEndsAtRow, NaiveTime);
