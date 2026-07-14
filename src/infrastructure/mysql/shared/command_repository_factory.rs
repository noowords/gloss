use std::sync::{ Arc };

use crate::domain::{
    shared::{ CommandRepositoryFactory },
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
pub struct MySqlCommandRepositoryFactory;

impl MySqlCommandRepositoryFactory {
    pub fn new() -> Self {
        Self::default()
    }
}

impl CommandRepositoryFactory for MySqlCommandRepositoryFactory {
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
