use super::value_objects::{ OtpId, OtpProviderType, OtpProviderKey, OtpCode };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Otp {
    id: OtpId,
    provider_type: OtpProviderType,
    provider_key: OtpProviderKey,
    code: OtpCode
}

impl Otp {
    pub fn create(
        provider_type: OtpProviderType,
        provider_key: OtpProviderKey,
        code: OtpCode
    ) -> Self {
        Self {
            id: OtpId::generate(),
            provider_type,
            provider_key,
            code
        }
    }

    pub fn restore(
        id: OtpId,
        provider_type: OtpProviderType,
        provider_key: OtpProviderKey,
        code: OtpCode
    ) -> Self {
        Self {
            id,
            provider_type,
            provider_key,
            code
        }
    }

    pub fn id(&self) -> OtpId {
        self.id
    }

    pub fn provider_type(&self) -> OtpProviderType {
        self.provider_type
    }

    pub fn provider_key(&self) -> OtpProviderKey {
        self.provider_key.clone()
    }

    pub fn code(&self) -> OtpCode {
        self.code.clone()
    }
}
