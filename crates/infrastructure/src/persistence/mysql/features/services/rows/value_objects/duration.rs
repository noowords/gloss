use domain::aggregates::service::value_objects::{ ServiceDuration };

#[derive(Debug, Copy, Clone, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
#[sqlx(transparent)]
pub struct MySqlServiceDurationRow(u32);

impl From<MySqlServiceDurationRow> for u32 {
    fn from(row: MySqlServiceDurationRow) -> Self {
        row.0
    }
}

impl From<u32> for MySqlServiceDurationRow {
    fn from(value: u32) -> Self {
        Self(value)
    }
}

impl From<MySqlServiceDurationRow> for ServiceDuration {
    fn from(row: MySqlServiceDurationRow) -> Self {
        row.0.into()
    }
}

impl From<ServiceDuration> for MySqlServiceDurationRow {
    fn from(entity: ServiceDuration) -> Self {
        Self(entity.into())
    }
}
