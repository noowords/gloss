#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OtpChallengeCodeHash(Vec<u8>);

impl From<OtpChallengeCodeHash> for Vec<u8> {
    fn from(value: OtpChallengeCodeHash) -> Self {
        value.0
    }
}

impl TryFrom<Vec<u8>> for OtpChallengeCodeHash {
    type Error = anyhow::Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        if value.len() > 64 {
            anyhow::bail!("Invalid OtpChallengeCodeHash");
        }

        Ok(Self(value))
    }
}
