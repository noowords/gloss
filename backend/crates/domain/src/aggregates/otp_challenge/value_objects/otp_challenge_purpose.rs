#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum OtpChallengePurpose {
    Authentication,
    PhoneVerification
}
