use domain::aggregates::service::value_objects::{ ServiceDescription };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
#[sqlx(transparent)]
pub struct MySqlServiceDescriptionRow(String);

impl From<MySqlServiceDescriptionRow> for String {
    fn from(row: MySqlServiceDescriptionRow) -> Self {
        row.0
    }
}

impl From<String> for MySqlServiceDescriptionRow {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl From<MySqlServiceDescriptionRow> for ServiceDescription {
    fn from(row: MySqlServiceDescriptionRow) -> Self {
        row.0.into()
    }
}

impl From<ServiceDescription> for MySqlServiceDescriptionRow {
    fn from(entity: ServiceDescription) -> Self {
        Self(entity.into())
    }
}
