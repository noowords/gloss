mod id;
mod category;
mod name;
mod description;
mod cover_url;
mod price;
mod duration;
mod is_active;

pub use id::{ ServiceId };
pub use category::{ ServiceCategory };
pub use name::{ ServiceName };
pub use description::{ ServiceDescription };
pub use cover_url::{ ServiceCoverUrl };
pub use price::{ ServicePrice };
pub use duration::{ ServiceDuration };
pub use is_active::{ ServiceIsActive };
