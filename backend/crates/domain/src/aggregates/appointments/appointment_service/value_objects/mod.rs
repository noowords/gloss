mod role;
mod service_name_snapshot;
mod price_snapshot;
mod duration_minutes_snapshot;

pub use role::{ AppointmentServiceRole };
pub use service_name_snapshot::{ AppointmentServiceNameSnapshot };
pub use price_snapshot::{ AppointmentServicePriceSnapshot };
pub use duration_minutes_snapshot::{ AppointmentServiceDurationMinutesSnapshot };
