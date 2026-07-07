use serde::{ Deserialize };
use uuid::{ Uuid };

#[derive(Deserialize)]
pub struct GetUserByIdQuery {
    pub id: Uuid
}
