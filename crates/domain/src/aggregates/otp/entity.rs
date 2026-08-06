use super::value_objects::{ OtpId, OtpPhone, OtpCode };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Otp {
    id: OtpId,
    phone: OtpPhone,
    code: OtpCode
}

impl Otp {
    pub fn create(
        phone: OtpPhone,
        code: OtpCode
    ) -> Self {
        Self {
            id: OtpId::generate(),
            phone,
            code
        }
    }
    
    pub fn restore(
        id: OtpId,
        phone: OtpPhone,
        code: OtpCode
    ) -> Self {
        Self {
            id,
            phone,
            code
        }
    }

    pub fn id(&self) -> OtpId {
        self.id
    }

    pub fn phone(&self) -> OtpPhone {
        self.phone.clone()
    }

    pub fn code(&self) -> OtpCode {
        self.code.clone()
    }
}
