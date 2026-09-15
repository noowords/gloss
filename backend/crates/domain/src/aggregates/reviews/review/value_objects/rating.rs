#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ReviewRating(u8);

impl From<ReviewRating> for u8 {
    fn from(value: ReviewRating) -> Self {
        value.0
    }
}

impl TryFrom<u8> for ReviewRating {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if !(1..=5).contains(&value) {
            anyhow::bail!("Invalid ReviewRating");
        }

        Ok(Self(value))
    }
}
