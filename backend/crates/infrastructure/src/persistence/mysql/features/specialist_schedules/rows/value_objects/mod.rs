use chrono::NaiveDate;
use uuid::Uuid;

crate::mysql_row_value!(MySqlSpecialistScheduleIdRow, Uuid);
crate::mysql_row_value!(MySqlSpecialistScheduleSpecialistIdRow, Uuid);
crate::mysql_row_value!(MySqlSpecialistScheduleEffectiveFromRow, NaiveDate);
crate::mysql_row_value!(MySqlSpecialistScheduleEffectiveUntilRow, NaiveDate);
