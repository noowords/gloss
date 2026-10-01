use chrono::NaiveTime;

use super::super::{
    errors::SpecialistTimeOffError,
    value_objects::{
        SpecialistId,
        SpecialistTimeOffId,
        SpecialistTimeOffDate,
        SpecialistTimeOffType,
        SpecialistTimeOffStartsAt,
        SpecialistTimeOffEndsAt,
        SpecialistTimeOffChargedLeaveMinutes,
        SpecialistTimeOffReason,
    },
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpecialistTimeOff {
    id: SpecialistTimeOffId,
    specialist_id: SpecialistId,
    date: SpecialistTimeOffDate,
    r#type: SpecialistTimeOffType,
    starts_at: Option<SpecialistTimeOffStartsAt>,
    ends_at: Option<SpecialistTimeOffEndsAt>,
    charged_leave_minutes: SpecialistTimeOffChargedLeaveMinutes,
    reason: Option<SpecialistTimeOffReason>
}

// MARK: Constructors
impl SpecialistTimeOff {
    pub fn create(
        specialist_id: SpecialistId,
        date: SpecialistTimeOffDate,
        r#type: SpecialistTimeOffType,
        starts_at: Option<SpecialistTimeOffStartsAt>,
        ends_at: Option<SpecialistTimeOffEndsAt>,
        charged_leave_minutes: SpecialistTimeOffChargedLeaveMinutes,
        reason: Option<SpecialistTimeOffReason>
    ) -> Result<Self, SpecialistTimeOffError> {
        Self::validate_interval(starts_at, ends_at)?;
        
        let id = SpecialistTimeOffId::generate();

        Ok(Self { id, specialist_id, date, r#type, starts_at, ends_at, charged_leave_minutes, reason })
    }

    pub fn restore(
        id: SpecialistTimeOffId,
        specialist_id: SpecialistId,
        date: SpecialistTimeOffDate,
        r#type: SpecialistTimeOffType,
        starts_at: Option<SpecialistTimeOffStartsAt>,
        ends_at: Option<SpecialistTimeOffEndsAt>,
        charged_leave_minutes: SpecialistTimeOffChargedLeaveMinutes,
        reason: Option<SpecialistTimeOffReason>
    ) -> Result<Self, SpecialistTimeOffError> {
        Self::validate_interval(starts_at, ends_at)?;
        
        Ok(Self { id, specialist_id, date, r#type, starts_at, ends_at, charged_leave_minutes, reason })
    }
}

// MARK: Validation
impl SpecialistTimeOff {
    fn validate_interval(
        starts_at: Option<SpecialistTimeOffStartsAt>,
        ends_at: Option<SpecialistTimeOffEndsAt>,
    ) -> Result<(), SpecialistTimeOffError> {
        match (starts_at, ends_at) {
            (None, None) => Ok(()),

            (Some(starts_at), Some(ends_at)) => {
                let starts_at: NaiveTime = starts_at.into();
                let ends_at: NaiveTime = ends_at.into();

                if starts_at >= ends_at {
                    return Err(SpecialistTimeOffError::InvalidInterval);
                }

                Ok(())
            }

            _ => Err(SpecialistTimeOffError::IncompleteInterval),
        }
    }
}

// MARK: Getters
impl SpecialistTimeOff {
    pub fn id(&self) -> SpecialistTimeOffId {
        self.id
    }

    pub fn specialist_id(&self) -> SpecialistId {
        self.specialist_id
    }

    pub fn date(&self) -> SpecialistTimeOffDate {
        self.date
    }

    pub fn r#type(&self) -> SpecialistTimeOffType {
        self.r#type
    }

    pub fn starts_at(&self) -> Option<SpecialistTimeOffStartsAt> {
        self.starts_at
    }

    pub fn ends_at(&self) -> Option<SpecialistTimeOffEndsAt> {
        self.ends_at
    }

    pub fn charged_leave_minutes(&self) -> SpecialistTimeOffChargedLeaveMinutes {
        self.charged_leave_minutes
    }

    pub fn reason(&self) -> Option<&SpecialistTimeOffReason> {
        self.reason.as_ref()
    }

    pub fn is_full_day(&self) -> bool {
        self.starts_at.is_none() && self.ends_at.is_none()
    }
}
