mod id;
mod category;
mod name;
mod description;
mod cover_url;
mod price;
mod duration;
mod is_active;

pub use id::{ MySqlServiceIdRow };
pub use category::{ MySqlServiceCategoryRow };
pub use name::{ MySqlServiceNameRow };
pub use description::{ MySqlServiceDescriptionRow };
pub use cover_url::{ MySqlServiceCoverUrlRow };
pub use price::{ MySqlServicePriceRow };
pub use duration::{ MySqlServiceDurationRow };
pub use is_active::{ MySqlServiceIsActiveRow };
