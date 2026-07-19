use domain::models::{
    user::{ UserRepository },
    profile::{ ProfileRepository },
    master::{ MasterRepository },
    appointment::{ AppointmentRepository }
};

use super::{ TxContext };

pub trait RepositoryFactory: Send + Sync {
    fn users<'a>(&'a self, ctx: &'a mut dyn TxContext) -> Result<Box<dyn UserRepository + 'a>, anyhow::Error>;

    fn profiles<'a>(&'a self, ctx: &'a mut dyn TxContext) -> Result<Box<dyn ProfileRepository + 'a>, anyhow::Error>;

    fn masters<'a>(&'a self, ctx: &'a mut dyn TxContext) -> Result<Box<dyn MasterRepository + 'a>, anyhow::Error>;
    
    fn appointments<'a>(&'a self, ctx: &'a mut dyn TxContext) -> Result<Box<dyn AppointmentRepository + 'a>, anyhow::Error>;
}
