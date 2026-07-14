use serde::{ Serialize };
use sqlx::{ FromRow };

use super::super::get_by_id::{ GetUserByIdView };

#[derive(Debug, Clone, FromRow, Serialize)]
pub struct GetUsersView {
    pub users: Vec<GetUserByIdView>
}
