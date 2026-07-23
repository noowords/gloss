use uuid::{ Uuid };
use sqlx::{ Type };

use domain::aggregates::appointment::value_objects::{ AppointmentId };

#[derive(Clone, Type)]
#[sqlx(transparent)]
pub struct MySqlAppointmentIdModel(Uuid);

impl MySqlAppointmentIdModel {
    pub fn new(value: Uuid) -> Self {
        Self(value)
    }

    pub fn value(&self) -> Uuid {
        self.0.clone()
    }
}

impl TryFrom<MySqlAppointmentIdModel> for AppointmentId {
    type Error = anyhow::Error;

    fn try_from(model: MySqlAppointmentIdModel) -> Result<Self, Self::Error> {
        Ok(Self::from(model.value()))
    }
}

impl From<AppointmentId> for MySqlAppointmentIdModel {
    fn from(id: AppointmentId) -> Self {
        Self(id.uuid())
    }
}
