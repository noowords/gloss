use domain::aggregates::services::service::value_objects::{ ServiceDurationMinutes };

#[derive(Debug, Copy, Clone, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
#[sqlx(transparent)]
pub struct MySqlServiceDurationMinutesRow(u16);

impl From<MySqlServiceDurationMinutesRow> for u16 {
    fn from(row: MySqlServiceDurationMinutesRow) -> Self {
        row.0
    }
}

impl From<u16> for MySqlServiceDurationMinutesRow {
    fn from(value: u16) -> Self {
        Self(value)
    }
}

impl TryFrom<MySqlServiceDurationMinutesRow> for ServiceDurationMinutes {
    type Error = anyhow::Error;

    fn try_from(row: MySqlServiceDurationMinutesRow) -> Result<Self, Self::Error> {
        row.0.try_into()
    }
}

impl From<ServiceDurationMinutes> for MySqlServiceDurationMinutesRow {
    fn from(entity: ServiceDurationMinutes) -> Self {
        Self(entity.into())
    }
}
