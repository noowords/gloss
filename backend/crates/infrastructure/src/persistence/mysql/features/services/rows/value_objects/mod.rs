mod id;
mod category;
mod name;
mod description;
mod preview_url;
mod kind;
mod duration_minutes;
mod is_active;

pub use id::{ MySqlServiceIdRow };
pub use category::{ MySqlServiceCategoryRow };
pub use name::{ MySqlServiceNameRow };
pub use description::{ MySqlServiceDescriptionRow };
pub use preview_url::{ MySqlServicePreviewUrlRow };
pub use kind::{ MySqlServiceKindRow };
pub use duration_minutes::{ MySqlServiceDurationMinutesRow };
pub use is_active::{ MySqlServiceIsActiveRow };

mod legacy_price;
mod legacy_duration;
pub use legacy_price::{ MySqlServicePriceRow };
pub use legacy_duration::{ MySqlServiceDurationRow };
