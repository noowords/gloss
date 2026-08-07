use domain::aggregates::service::value_objects::{ ServiceName };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlServiceNameRow(String);

impl From<MySqlServiceNameRow> for ServiceName {
    fn from(model: MySqlServiceNameRow) -> Self {
        model.0.into()
    }
}

impl From<ServiceName> for MySqlServiceNameRow {
    fn from(entity: ServiceName) -> Self {
        Self(entity.into())
    }
}
