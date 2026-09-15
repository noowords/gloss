use chrono::NaiveDateTime;
use uuid::Uuid;

crate::mysql_row_value!(MySqlUserProviderIdRow, Uuid);
crate::mysql_row_value!(MySqlUserProviderUserIdRow, Uuid);
crate::mysql_row_value!(MySqlUserProviderProviderRow, String);
crate::mysql_row_value!(MySqlUserProviderSubjectRow, String);
crate::mysql_row_value!(MySqlUserProviderVerifiedAtRow, NaiveDateTime);
