use std::sync::{ Arc };

use crate::domain::{
    shared::{ InfrastructureFactory },
    models::{
        user::{ UserRepository },
        profile::{ ProfileRepository },
        master::{ MasterRepository },
        appointment::{ AppointmentRepository }
    }
};

use super::super::models::{
    user::{ MySqlUserRepository },
    profile::{ MySqlProfileRepository },
    master::{ MySqlMasterRepository },
    appointment::{ MySqlAppointmentRepository }
};

#[derive(Default)]
pub struct MySqlInfrastructureFactory;

impl MySqlInfrastructureFactory {
    pub fn new() -> Self {
        Self::default()
    }
}

impl InfrastructureFactory for MySqlInfrastructureFactory {
    fn user_repository(&self) -> Arc<dyn UserRepository> {
        Arc::new(MySqlUserRepository::new())
    }

    fn profile_repository(&self) -> Arc<dyn ProfileRepository> {
        Arc::new(MySqlProfileRepository::new())
    }

    fn master_repository(&self) -> Arc<dyn MasterRepository> {
        Arc::new(MySqlMasterRepository::new())
    }

    fn appointment_repository(&self) -> Arc<dyn AppointmentRepository> {
        Arc::new(MySqlAppointmentRepository::new())
    }
}
