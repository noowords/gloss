use std::sync::{ Arc };

use crate::domain::models::{
    user::{ UserRepository },
    profile::{ ProfileRepository },
    master::{ MasterRepository },
    appointment::{ AppointmentRepository }
};

pub trait RepositoryFactory: Send + Sync {
    fn users_repository(&self) -> Arc<dyn UserRepository>;

    fn profiles_repository(&self) -> Arc<dyn ProfileRepository>;

    fn masters_repository(&self) -> Arc<dyn MasterRepository>;
    
    fn appointments_repository(&self) -> Arc<dyn AppointmentRepository>;
}
