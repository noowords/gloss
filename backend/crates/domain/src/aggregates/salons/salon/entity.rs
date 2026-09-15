use super::value_objects::{ SalonId, SalonCode, SalonName, SalonCity, SalonAddress, SalonTimezone, SalonStatus };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Salon {
    id: SalonId,
    code: SalonCode,
    name: SalonName,
    city: SalonCity,
    address: Option<SalonAddress>,
    timezone: SalonTimezone,
    status: SalonStatus
}

impl Salon {
    pub fn create(
        code: SalonCode,
        name: SalonName,
        city: SalonCity,
        address: Option<SalonAddress>,
        timezone: SalonTimezone
    ) -> Result<Self, anyhow::Error> {
        let status = SalonStatus::try_from("active")?;
        let id = SalonId::generate();
        Self::restore(
            id,
            code,
            name,
            city,
            address,
            timezone,
            status
        )
    }

    pub fn restore(
        id: SalonId,
        code: SalonCode,
        name: SalonName,
        city: SalonCity,
        address: Option<SalonAddress>,
        timezone: SalonTimezone,
        status: SalonStatus
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            id,
            code,
            name,
            city,
            address,
            timezone,
            status
        })
    }

    pub fn id(&self) -> SalonId {
        self.id
    }

    pub fn code(&self) -> SalonCode {
        self.code.clone()
    }

    pub fn name(&self) -> SalonName {
        self.name.clone()
    }

    pub fn city(&self) -> SalonCity {
        self.city.clone()
    }

    pub fn address(&self) -> Option<SalonAddress> {
        self.address.clone()
    }

    pub fn timezone(&self) -> SalonTimezone {
        self.timezone.clone()
    }

    pub fn status(&self) -> SalonStatus {
        self.status.clone()
    }
}
