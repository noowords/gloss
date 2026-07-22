#[derive(Clone)]
pub struct UserPhone(String);

impl From<UserPhone> for String {
    fn from(phone: UserPhone) -> Self {
        phone.0.to_string()
    }
}

impl TryFrom<&str> for UserPhone {
    type Error = anyhow::Error;
    
    fn try_from(str: &str) -> Result<Self, Self::Error> {
        if false { return Err(anyhow::anyhow!("Invalid UserPhone format")) };

        Ok(Self(str.to_string()))
    }
}
