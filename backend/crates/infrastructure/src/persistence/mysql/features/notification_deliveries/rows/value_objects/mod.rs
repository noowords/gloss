use chrono::NaiveDateTime;
use uuid::Uuid;

crate::mysql_row_value!(MySqlNotificationDeliveryIdRow, Uuid);
crate::mysql_row_value!(MySqlNotificationDeliveryNotificationIdRow, Uuid);
crate::mysql_row_value!(MySqlNotificationDeliveryChannelRow, String);
crate::mysql_row_value!(MySqlNotificationDeliveryStatusRow, String);
crate::mysql_row_value!(MySqlNotificationDeliveryAttemptedAtRow, NaiveDateTime);
crate::mysql_row_value!(MySqlNotificationDeliveryDeliveredAtRow, NaiveDateTime);
crate::mysql_row_value!(MySqlNotificationDeliveryReadAtRow, NaiveDateTime);
crate::mysql_row_value!(MySqlNotificationDeliveryErrorRow, String);
