use domain::aggregates::{
    users::user::value_objects::{ UserId },
    users::user_role::value_objects::{ UserRoleName as UserRole }
};

pub trait TokenService: Send + Sync + 'static {
    fn generate_access_token(&self, user_id: UserId, role: UserRole) -> Result<String, anyhow::Error>;
    
    fn generate_refresh_token(&self, user_id: UserId) -> Result<String, anyhow::Error>;

    fn verify_access_token(&self, token: &str) -> Result<(UserId, UserRole), anyhow::Error>;
    
    fn verify_refresh_token(&self, token: &str) -> Result<UserId, anyhow::Error>;
}
