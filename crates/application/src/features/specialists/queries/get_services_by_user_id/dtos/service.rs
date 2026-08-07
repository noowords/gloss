use uuid::{ Uuid };
use bigdecimal::{ BigDecimal };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Service {
    pub id: Uuid,
    pub specialist_id: Uuid,
    pub name: String,
    pub price: BigDecimal,
    pub duration: u32,
    pub is_active: bool
}
