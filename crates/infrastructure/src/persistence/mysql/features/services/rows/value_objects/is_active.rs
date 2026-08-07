use domain::aggregates::service::value_objects::{ ServiceIsActive };

#[derive(Debug, Copy, Clone, PartialEq, Eq, sqlx::Type)]
#[sqlx(transparent)]
pub struct MySqlServiceIsActiveRow(bool);

impl From<MySqlServiceIsActiveRow> for ServiceIsActive {
    fn from(model: MySqlServiceIsActiveRow) -> Self {
        model.0.into()
    }
}

impl From<ServiceIsActive> for MySqlServiceIsActiveRow {
    fn from(entity: ServiceIsActive) -> Self {
        Self(entity.into())
    }
}
