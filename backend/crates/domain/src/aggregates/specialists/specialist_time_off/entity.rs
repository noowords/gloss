use crate::aggregates::specialists::specialist::value_objects::{ SpecialistId };

use super::value_objects::{ SpecialistTimeOffId, SpecialistTimeOffDate, SpecialistTimeOffType, SpecialistTimeOffStartsAt, SpecialistTimeOffEndsAt, SpecialistTimeOffChargedLeaveMinutes, SpecialistTimeOffReason };

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

impl SpecialistTimeOff {
    pub fn create(
        specialist_id: SpecialistId,
        date: SpecialistTimeOffDate,
        r#type: SpecialistTimeOffType,
        starts_at: Option<SpecialistTimeOffStartsAt>,
        ends_at: Option<SpecialistTimeOffEndsAt>,
        reason: Option<SpecialistTimeOffReason>
    ) -> Result<Self, anyhow::Error> {
        let charged_leave_minutes = SpecialistTimeOffChargedLeaveMinutes::try_from(0u16)?;
        let id = SpecialistTimeOffId::generate();
        Self::restore(
            id,
            specialist_id,
            date,
            r#type,
            starts_at,
            ends_at,
            charged_leave_minutes,
            reason
        )
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
    ) -> Result<Self, anyhow::Error> {
        match (starts_at, ends_at) {
            (None, None) => {},
            (Some(start), Some(end)) if chrono::NaiveTime::from(start) < chrono::NaiveTime::from(end) => {},
            _ => anyhow::bail!("Time off must have both interval boundaries or neither")
        }

        Ok(Self {
            id,
            specialist_id,
            date,
            r#type,
            starts_at,
            ends_at,
            charged_leave_minutes,
            reason
        })
    }

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
        self.r#type.clone()
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

    pub fn reason(&self) -> Option<SpecialistTimeOffReason> {
        self.reason.clone()
    }
}
