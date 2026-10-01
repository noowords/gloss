use std::net::IpAddr;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct AuthSessionIpAddress(IpAddr);

// MARK: Conversions
impl From<IpAddr> for AuthSessionIpAddress {
    fn from(value: IpAddr) -> Self {
        Self(value)
    }
}

impl From<AuthSessionIpAddress> for IpAddr {
    fn from(vo: AuthSessionIpAddress) -> Self {
        vo.0
    }
}
