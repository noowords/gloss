use super::value_objects::{ OtpId, OtpProviderType, OtpProviderKey, OtpCode, OtpExpiresAt };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Otp {
    id: OtpId,
    provider_type: OtpProviderType,
    provider_key: OtpProviderKey,
    code: OtpCode,
    expires_at: OtpExpiresAt
}

impl Otp {
    pub fn generate(
        provider_type: OtpProviderType,
        provider_key: OtpProviderKey
    ) -> Self {
        Self {
            id: OtpId::generate(),
            provider_type,
            provider_key,
            code: OtpCode::generate_random_numeric(6),
            expires_at: OtpExpiresAt::in_minutes(10)
        }
    }

    pub fn restore(
        id: OtpId,
        provider_type: OtpProviderType,
        provider_key: OtpProviderKey,
        code: OtpCode,
        expires_at: OtpExpiresAt
    ) -> Self {
        Self {
            id,
            provider_type,
            provider_key,
            code,
            expires_at
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
