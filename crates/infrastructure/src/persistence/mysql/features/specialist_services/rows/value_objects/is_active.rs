use domain::aggregates::specialist_service::value_objects::{ SpecialistServiceIsActive };

#[derive(Debug, Copy, Clone, PartialEq, Eq, sqlx::Type, sqlx::FromRow)]
#[sqlx(transparent)]
pub struct MySqlSpecialistServiceIsActiveRow(bool);

impl From<MySqlSpecialistServiceIsActiveRow> for bool {
    fn from(row: MySqlSpecialistServiceIsActiveRow) -> Self {
        row.0
    }
}

impl From<bool> for MySqlSpecialistServiceIsActiveRow {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl From<MySqlSpecialistServiceIsActiveRow> for SpecialistServiceIsActive {
    fn from(row: MySqlSpecialistServiceIsActiveRow) -> Self {
        row.0.into()
    }
}

impl From<SpecialistServiceIsActive> for MySqlSpecialistServiceIsActiveRow {
    fn from(entity: SpecialistServiceIsActive) -> Self {
        Self(entity.into())
    }
}
