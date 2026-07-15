use async_trait::{ async_trait };

use crate::application::common::persistence::{ TxContext };

use super::{
    Appointment,
    value_objects::{ AppointmentId }
};

#[async_trait]
pub trait AppointmentRepository: Send + Sync {
    async fn create(
        &self,
        ctx: &mut dyn TxContext,
        appointment: &Appointment
    ) -> Result<(), anyhow::Error>;
    
    async fn get_by_id(
        &self,
        ctx: &mut dyn TxContext,
        id: AppointmentId
    ) -> Result<Option<Appointment>, anyhow::Error>;
    
    async fn exists(
        &self,
        ctx: &mut dyn TxContext,
        id: AppointmentId
    ) -> Result<bool, anyhow::Error>;
    
    async fn update(
        &self,
        ctx: &mut dyn TxContext,
        appointment: &Appointment
    ) -> Result<(), anyhow::Error>;
    
    async fn remove(
        &self,
        ctx: &mut dyn TxContext,
        id: AppointmentId
    ) -> Result<(), anyhow::Error>;
}
