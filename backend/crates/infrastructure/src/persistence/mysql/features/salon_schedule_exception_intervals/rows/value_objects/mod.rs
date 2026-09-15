use chrono::NaiveTime;
use uuid::Uuid;

crate::mysql_row_value!(MySqlSalonScheduleExceptionIntervalIdRow, Uuid);
crate::mysql_row_value!(MySqlSalonScheduleExceptionIntervalExceptionIdRow, Uuid);
crate::mysql_row_value!(MySqlSalonScheduleExceptionIntervalStartsAtRow, NaiveTime);
crate::mysql_row_value!(MySqlSalonScheduleExceptionIntervalEndsAtRow, NaiveTime);
