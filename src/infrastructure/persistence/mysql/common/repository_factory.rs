use std::sync::{ Arc };

use crate::domain::{
    common::{ RepositoryFactory },
    models::{
        user::{ UserRepository },
        profile::{ ProfileRepository },
        master::{ MasterRepository },
        appointment::{ AppointmentRepository }
    }
};

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
    fn users_repository(&self) -> Arc<dyn UserRepository> {
        Arc::new(MySqlUserRepository::new())
    }

    fn profiles_repository(&self) -> Arc<dyn ProfileRepository> {
        Arc::new(MySqlProfileRepository::new())
    }

    fn masters_repository(&self) -> Arc<dyn MasterRepository> {
        Arc::new(MySqlMasterRepository::new())
    }

    fn appointments_repository(&self) -> Arc<dyn AppointmentRepository> {
        Arc::new(MySqlAppointmentRepository::new())
    }
}
