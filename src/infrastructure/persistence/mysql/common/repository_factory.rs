use crate::domain::models::{
    user::{ UserRepository },
    profile::{ ProfileRepository },
    master::{ MasterRepository },
    appointment::{ AppointmentRepository }
};
use crate::application::common::persistence::{ TxContext, RepositoryFactory };

use super::super::repositories::{
    MySqlUserRepository,
    MySqlProfileRepository,
    MySqlMasterRepository,
    MySqlAppointmentRepository
};

use super::{ MySqlTxContext };

#[derive(Default)]
pub struct MySqlRepositoryFactory;

impl MySqlRepositoryFactory {
    pub fn new() -> Self {
        Self::default()
    }
}

impl RepositoryFactory for MySqlRepositoryFactory {
    fn users<'a>(&self, ctx: &'a mut dyn TxContext) -> Result<Box<dyn UserRepository + 'a>, anyhow::Error> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| anyhow::anyhow!("Invalid TxContext context".to_string()))?;

        Ok(Box::new(MySqlUserRepository::new(&mut ctx.tx)))
    }

    fn profiles<'a>(&self, ctx: &'a mut dyn TxContext) -> Result<Box<dyn ProfileRepository + 'a>, anyhow::Error> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| anyhow::anyhow!("Invalid TxContext context".to_string()))?;

        Ok(Box::new(MySqlProfileRepository::new(&mut ctx.tx)))
    }

    fn masters<'a>(&self, ctx: &'a mut dyn TxContext) -> Result<Box<dyn MasterRepository + 'a>, anyhow::Error> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| anyhow::anyhow!("Invalid TxContext context".to_string()))?;

        Ok(Box::new(MySqlMasterRepository::new(&mut ctx.tx)))
    }

    fn appointments<'a>(&self, ctx: &'a mut dyn TxContext) -> Result<Box<dyn AppointmentRepository + 'a>, anyhow::Error> {
        let ctx = ctx
            .downcast_mut::<MySqlTxContext>()
            .ok_or_else(|| anyhow::anyhow!("Invalid TxContext context".to_string()))?;

        Ok(Box::new(MySqlAppointmentRepository::new(&mut ctx.tx)))
    }
}
