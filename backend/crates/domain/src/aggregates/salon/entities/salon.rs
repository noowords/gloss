use super::super::value_objects::{
    SalonId,
    SalonCode,
    SalonName,
    SalonCity,
    SalonAddress,
    SalonTimezone,
    SalonStatus
};

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

// MARK: Constructors
impl Salon {
    pub fn create(
        code: SalonCode,
        name: SalonName,
        city: SalonCity,
        address: Option<SalonAddress>,
        timezone: SalonTimezone
    ) -> Self {
        let id = SalonId::generate();
        let status = SalonStatus::Active;
        
        Self { id, code, name, city, address, timezone, status }
    }

    pub fn restore(
        id: SalonId,
        code: SalonCode,
        name: SalonName,
        city: SalonCity,
        address: Option<SalonAddress>,
        timezone: SalonTimezone,
        status: SalonStatus
    ) -> Self {
        Self { id, code, name, city, address, timezone, status }
    }
}

// MARK: Behavior
impl Salon {
    pub fn change_code(&mut self, code: SalonCode) {
        self.code = code;
    }

    pub fn change_name(&mut self, name: SalonName) {
        self.name = name;
    }

    pub fn change_city(&mut self, city: SalonCity) {
        self.city = city;
    }

    pub fn change_address(&mut self, address: Option<SalonAddress>) {
        self.address = address;
    }

    pub fn change_timezone(&mut self, timezone: SalonTimezone) {
        self.timezone = timezone;
    }

    pub fn activate(&mut self) {
        self.status = SalonStatus::Active;
    }

    pub fn deactivate(&mut self) {
        self.status = SalonStatus::Inactive;
    }
}

// MARK: Getters
impl Salon {
    pub fn id(&self) -> SalonId {
        self.id
    }

    pub fn code(&self) -> &SalonCode {
        &self.code
    }

    pub fn name(&self) -> &SalonName {
        &self.name
    }

    pub fn city(&self) -> &SalonCity {
        &self.city
    }

    pub fn address(&self) -> Option<&SalonAddress> {
        self.address.as_ref()
    }

    pub fn timezone(&self) -> SalonTimezone {
        self.timezone
    }

    pub fn status(&self) -> SalonStatus {
        self.status
    }

    pub fn is_active(&self) -> bool {
        self.status == SalonStatus::Active
    }
}
