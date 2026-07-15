use async_trait::{ async_trait };

use super::{
    Appointment,
    value_objects::{ AppointmentId }
};

#[async_trait]
pub trait AppointmentRepository: Send + Sync {
    async fn create(&self, appointment: &Appointment) -> Result<(), anyhow::Error>;
    
    async fn get_by_id(&self, id: AppointmentId) -> Result<Option<Appointment>, anyhow::Error>;
    
    async fn exists(&self, id: AppointmentId) -> Result<bool, anyhow::Error>;
    
    async fn update(&self, appointment: &Appointment) -> Result<(), anyhow::Error>;
    
    async fn remove(&self, id: AppointmentId) -> Result<(), anyhow::Error>;
}
