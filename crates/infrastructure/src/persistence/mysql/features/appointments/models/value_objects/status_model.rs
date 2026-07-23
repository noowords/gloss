use sqlx::{ Type };

use domain::aggregates::appointment::value_objects::{ AppointmentStatus };

#[derive(Clone, Type)]
#[sqlx(transparent)]
pub struct MySqlAppointmentStatusModel(String);

impl MySqlAppointmentStatusModel {
    pub fn new(value: String) -> Self {
        Self(value)
    }

    pub fn value(&self) -> String {
        self.0.clone()
    }
}

impl TryFrom<MySqlAppointmentStatusModel> for AppointmentStatus {
    type Error = anyhow::Error;

    fn try_from(model: MySqlAppointmentStatusModel) -> Result<Self, Self::Error> {
        AppointmentStatus::try_from(model.value().as_str())
    }
}

impl From<AppointmentStatus> for MySqlAppointmentStatusModel {
    fn from(role: AppointmentStatus) -> Self {
        Self::new(role.into())
    }
}
