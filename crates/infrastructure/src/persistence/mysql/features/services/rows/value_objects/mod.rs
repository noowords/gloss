mod id;
mod name;
mod price;
mod duration;
mod is_active;

pub use id::{ MySqlServiceIdRow };
pub use name::{ MySqlServiceNameRow };
pub use price::{ MySqlServicePriceRow };
pub use duration::{ MySqlServiceDurationRow };
pub use is_active::{ MySqlServiceIsActiveRow };
