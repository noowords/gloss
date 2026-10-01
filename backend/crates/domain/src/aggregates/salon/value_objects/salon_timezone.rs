use chrono_tz::Tz;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SalonTimezone(Tz);

// MARK: Conversions
impl From<Tz> for SalonTimezone {
    fn from(value: Tz) -> Self {
        Self(value)
    }
}

impl From<SalonTimezone> for Tz {
    fn from(vo: SalonTimezone) -> Self {
        vo.0
    }
}
