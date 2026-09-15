use chrono::NaiveDate;
use uuid::Uuid;

crate::mysql_row_value!(MySqlSalonScheduleExceptionIdRow, Uuid);
crate::mysql_row_value!(MySqlSalonScheduleExceptionSalonIdRow, Uuid);
crate::mysql_row_value!(MySqlSalonScheduleExceptionDateRow, NaiveDate);
crate::mysql_row_value!(MySqlSalonScheduleExceptionTypeRow, String);
crate::mysql_row_value!(MySqlSalonScheduleExceptionReasonRow, String);
