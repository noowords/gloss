use chrono::NaiveDate;
use uuid::Uuid;

crate::mysql_row_value!(MySqlSpecialistIdRow, Uuid);
crate::mysql_row_value!(MySqlSpecialistSalonIdRow, Uuid);
crate::mysql_row_value!(MySqlSpecialistBioRow, String);
crate::mysql_row_value!(MySqlSpecialistExperienceStartedAtRow, NaiveDate);
crate::mysql_row_value!(MySqlSpecialistStatusRow, String);
