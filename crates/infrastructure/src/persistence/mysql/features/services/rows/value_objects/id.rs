use uuid::{ Uuid };

use domain::aggregates::service::value_objects::{ ServiceId };

#[derive(Debug, Copy, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlServiceIdRow(Uuid);

impl From<MySqlServiceIdRow> for ServiceId {
    fn from(model: MySqlServiceIdRow) -> Self {
        model.0.into()
    }
}

impl From<ServiceId> for MySqlServiceIdRow {
    fn from(entity: ServiceId) -> Self {
        Self(entity.into())
    }
}
