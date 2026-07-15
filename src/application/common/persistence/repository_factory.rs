use crate::domain::models::{
    user::{ UserRepository },
    profile::{ ProfileRepository },
    master::{ MasterRepository },
    appointment::{ AppointmentRepository }
};

use crate::application::common::persistence::{ TxContext };

pub trait RepositoryFactory: Send + Sync {
    fn user_repository<'a>(&'a self, ctx: &'a mut dyn TxContext) -> Result<Box<dyn UserRepository + 'a>, anyhow::Error>;

    fn profile_repository<'a>(&'a self, ctx: &'a mut dyn TxContext) -> Result<Box<dyn ProfileRepository + 'a>, anyhow::Error>;

    fn master_repository<'a>(&'a self, ctx: &'a mut dyn TxContext) -> Result<Box<dyn MasterRepository + 'a>, anyhow::Error>;
    
    fn appointment_repository<'a>(&'a self, ctx: &'a mut dyn TxContext) -> Result<Box<dyn AppointmentRepository + 'a>, anyhow::Error>;
}
