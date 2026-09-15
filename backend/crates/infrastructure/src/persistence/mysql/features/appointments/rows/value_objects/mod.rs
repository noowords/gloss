mod id;
mod date;
mod time;
mod duration;
mod status;

pub use id::{ MySqlAppointmentIdRow };
pub use date::{ MySqlAppointmentDateRow };
pub use time::{ MySqlAppointmentTimeRow };
pub use duration::{ MySqlAppointmentDurationRow };
pub use status::{ MySqlAppointmentStatusRow };

use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use uuid::Uuid;

crate::mysql_row_value!(MySqlAppointmentClientIdRow, Uuid);
crate::mysql_row_value!(MySqlAppointmentSalonIdRow, Uuid);
crate::mysql_row_value!(MySqlAppointmentSpecialistIdRow, Uuid);
crate::mysql_row_value!(MySqlAppointmentStartsAtRow, NaiveDateTime);
crate::mysql_row_value!(MySqlAppointmentEndsAtRow, NaiveDateTime);
crate::mysql_row_value!(MySqlAppointmentTotalPriceSnapshotRow, BigDecimal);
crate::mysql_row_value!(MySqlAppointmentTotalDurationMinutesSnapshotRow, u16);
crate::mysql_row_value!(MySqlAppointmentCancelledAtRow, NaiveDateTime);
crate::mysql_row_value!(MySqlAppointmentCancellationReasonRow, String);
