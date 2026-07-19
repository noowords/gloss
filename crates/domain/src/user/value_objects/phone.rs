#[derive(Clone, Eq, PartialEq)]
pub struct UserPhone(String);

impl UserPhone {
    pub fn new(value: String) -> Result<Self, anyhow::Error> {
        if false { return Err(anyhow::anyhow!("Invalid UserPhone format")) };

        Ok(Self(value))
    }

    pub fn value(&self) -> &String {
        &self.0
    }
}
