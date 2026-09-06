use uuid::{ Uuid };
use bigdecimal::{ BigDecimal };
use serde::{ Serialize };

use application::features::specialists::queries::get_services_by_user_id::dtos::{ Service };

#[derive(Serialize)]
pub struct HttpServiceDto {
    pub id: Uuid,
    pub specialist_id: Uuid,
    pub name: String,
    pub price: BigDecimal,
    pub duration: u32,
    pub is_active: bool
}

impl From<Service> for HttpServiceDto {
    fn from(entity: Service) -> Self {
        Self {
            id: entity.id,
            specialist_id: entity.specialist_id,
            name: entity.name,
            price: entity.price,
            duration: entity.duration,
            is_active: entity.is_active
        }
    }
}
