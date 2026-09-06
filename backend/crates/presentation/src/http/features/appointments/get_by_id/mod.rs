mod controller;
mod request;
mod response;
pub mod dtos;

pub use controller::{ get_by_id };
pub use request::{ GetAppointmentByIdRequest };
pub use response::{ GetAppointmentByIdResponse };
