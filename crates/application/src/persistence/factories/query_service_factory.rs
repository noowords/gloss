use std::sync::Arc;

use super::super::super::queries::users::{ UsersQueryService };

pub trait QueryServiceFactory: Send + Sync {
    fn users_service(&self) -> Arc<dyn UsersQueryService>;
}
