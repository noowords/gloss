use uuid::Uuid;

crate::mysql_row_value!(MySqlNotificationIdRow, Uuid);
crate::mysql_row_value!(MySqlNotificationUserIdRow, Uuid);
crate::mysql_row_value!(MySqlNotificationTypeRow, String);
crate::mysql_row_value!(MySqlNotificationTitleRow, String);
crate::mysql_row_value!(MySqlNotificationMessageRow, String);
