use domain::aggregates::service::value_objects::{ ServiceName };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlServiceNameRow(String);

impl From<MySqlServiceNameRow> for String {
    fn from(row: MySqlServiceNameRow) -> Self {
        row.0
    }
}

impl From<String> for MySqlServiceNameRow {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<MySqlServiceNameRow> for ServiceName {
    fn from(row: MySqlServiceNameRow) -> Self {
        row.0.into()
    }
}

impl From<ServiceName> for MySqlServiceNameRow {
    fn from(entity: ServiceName) -> Self {
        Self(entity.into())
    }
}
