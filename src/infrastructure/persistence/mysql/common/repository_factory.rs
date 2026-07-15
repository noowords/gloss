use std::sync::{ Arc };

use crate::domain::models::{
    user::{ UserRepository },
    profile::{ ProfileRepository },
    master::{ MasterRepository },
    appointment::{ AppointmentRepository }
};
use crate::application::common::persistence::{ RepositoryFactory };

use super::super::repositories::{
    MySqlUserRepository,
    MySqlProfileRepository,
    MySqlMasterRepository,
    MySqlAppointmentRepository
};

#[derive(Default)]
pub struct MySqlRepositoryFactory;

impl MySqlRepositoryFactory {
    pub fn new() -> Self {
        Self::default()
    }
}

impl RepositoryFactory for MySqlRepositoryFactory {
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
