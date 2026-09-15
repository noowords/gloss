mod price;
use uuid::Uuid;

crate::mysql_row_value!(MySqlAppointmentServiceAppointmentIdRow, Uuid);
crate::mysql_row_value!(MySqlAppointmentServiceServiceIdRow, Uuid);
crate::mysql_row_value!(MySqlAppointmentServiceRoleRow, String);
crate::mysql_row_value!(MySqlAppointmentServiceNameSnapshotRow, String);
crate::mysql_row_value!(MySqlAppointmentServiceDurationMinutesSnapshotRow, u16);

pub use price::{ MySqlAppointmentServicePriceSnapshotRow };
