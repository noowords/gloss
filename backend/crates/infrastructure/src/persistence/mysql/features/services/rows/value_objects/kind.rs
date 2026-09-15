use domain::aggregates::services::service::value_objects::{ ServiceKind };

#[derive(Debug, Clone, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
#[sqlx(transparent)]
pub struct MySqlServiceKindRow(String);

impl From<MySqlServiceKindRow> for String {
    fn from(row: MySqlServiceKindRow) -> Self {
        row.0
    }
}

impl From<String> for MySqlServiceKindRow {
    fn from(value: String) -> Self {
        Self(value)
    }
}

impl TryFrom<MySqlServiceKindRow> for ServiceKind {
    type Error = anyhow::Error;

    fn try_from(row: MySqlServiceKindRow) -> Result<Self, Self::Error> {
        row.0.try_into()
    }
}

impl From<ServiceKind> for MySqlServiceKindRow {
    fn from(entity: ServiceKind) -> Self {
        Self(entity.into())
    }
}
