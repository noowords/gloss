use chrono::NaiveDateTime;

use crate::aggregates::{
    user::value_objects::UserId,
    specialist::value_objects::SpecialistId,
    salon::value_objects::SalonId
};

use super::super::{
    errors::AppointmentError,
    value_objects::{
        AppointmentId,
        AppointmentStatus,
        AppointmentStartsAt,
        AppointmentEndsAt,
        AppointmentTotalPriceSnapshot,
        AppointmentTotalDurationMinutesSnapshot,
        AppointmentCancellationReason,
        AppointmentCreatedAt,
        AppointmentCancelledAt
    }
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Appointment {
    id: AppointmentId,
    client_id: UserId,
    salon_id: SalonId,
    specialist_id: SpecialistId,
    status: AppointmentStatus,
    starts_at: AppointmentStartsAt,
    ends_at: AppointmentEndsAt,
    total_price_snapshot: AppointmentTotalPriceSnapshot,
    total_duration_minutes_snapshot: AppointmentTotalDurationMinutesSnapshot,
    cancellation_reason: Option<AppointmentCancellationReason>,
    created_at: AppointmentCreatedAt,
    cancelled_at: Option<AppointmentCancelledAt>
}

// MARK: Constructors
impl Appointment {
    pub fn create(
        client_id: UserId,
        salon_id: SalonId,
        specialist_id: SpecialistId,
        starts_at: AppointmentStartsAt,
        ends_at: AppointmentEndsAt,
        total_price_snapshot: AppointmentTotalPriceSnapshot,
        total_duration_minutes_snapshot: AppointmentTotalDurationMinutesSnapshot,
        created_at: AppointmentCreatedAt
    ) -> Result<Self, AppointmentError> {
        Self::validate_time_range(starts_at, ends_at)?;
        Self::validate_duration(starts_at, ends_at, total_duration_minutes_snapshot)?;
        Self::validate_creation_time(created_at, starts_at)?;

        let id = AppointmentId::generate();
        let status = AppointmentStatus::Scheduled;
        let cancellation_reason = None;
        let cancelled_at = None;

        Ok(Self { id, client_id, salon_id, specialist_id, status, starts_at, ends_at, total_price_snapshot, total_duration_minutes_snapshot, cancellation_reason, created_at, cancelled_at })
    }

    pub fn restore(
        id: AppointmentId,
        client_id: UserId,
        salon_id: SalonId,
        specialist_id: SpecialistId,
        status: AppointmentStatus,
        ends_at: AppointmentEndsAt,
        created_at: AppointmentCreatedAt,
        total_price_snapshot: AppointmentTotalPriceSnapshot,
        total_duration_minutes_snapshot: AppointmentTotalDurationMinutesSnapshot,
        cancellation_reason: Option<AppointmentCancellationReason>,
        starts_at: AppointmentStartsAt,
        cancelled_at: Option<AppointmentCancelledAt>
    ) -> Result<Self, AppointmentError> {
        Self::validate_time_range(starts_at, ends_at)?;
        Self::validate_duration(starts_at, ends_at, total_duration_minutes_snapshot)?;
        Self::validate_creation_time(created_at, starts_at)?;
        Self::validate_cancellation_state(status, cancelled_at, cancellation_reason.as_ref())?;
        Self::validate_cancellation_time(created_at, cancelled_at)?;

        Ok(Self { id, client_id, salon_id, specialist_id, status, starts_at, ends_at, total_price_snapshot, total_duration_minutes_snapshot, cancellation_reason, created_at, cancelled_at })
    }
}

// MARK: Validation
impl Appointment {
    fn validate_time_range(
        starts_at: AppointmentStartsAt,
        ends_at: AppointmentEndsAt
    ) -> Result<(), AppointmentError> {
        let starts_at: NaiveDateTime = starts_at.into();
        let ends_at: NaiveDateTime = ends_at.into();

        if ends_at <= starts_at {
            return Err(AppointmentError::InvalidTimeRange);
        }

        Ok(())
    }

    fn validate_duration(
        starts_at: AppointmentStartsAt,
        ends_at: AppointmentEndsAt,
        total_duration_minutes_snapshot: AppointmentTotalDurationMinutesSnapshot
    ) -> Result<(), AppointmentError> {
        let starts_at: NaiveDateTime = starts_at.into();
        let ends_at: NaiveDateTime = ends_at.into();
        let duration_minutes: u16 = total_duration_minutes_snapshot.into();
    
        let actual_duration_minutes = (ends_at - starts_at).num_minutes();
    
        if actual_duration_minutes != i64::from(duration_minutes) {
            return Err(AppointmentError::InvalidDuration);
        }
    
        Ok(())
    }

    fn validate_creation_time(
        created_at: AppointmentCreatedAt,
        starts_at: AppointmentStartsAt
    ) -> Result<(), AppointmentError> {
        let created_at: NaiveDateTime = created_at.into();
        let starts_at: NaiveDateTime = starts_at.into();

        if starts_at <= created_at {
            return Err(AppointmentError::InvalidStartTime);
        }

        Ok(())
    }

    fn validate_cancellation_state(
        status: AppointmentStatus,
        cancelled_at: Option<AppointmentCancelledAt>,
        cancellation_reason: Option<&AppointmentCancellationReason>
    ) -> Result<(), AppointmentError> {
        match status {
            AppointmentStatus::Cancelled => {
                if cancelled_at.is_none() {
                    return Err(AppointmentError::MissingCancellationTime);
                }
            }

            AppointmentStatus::Scheduled
            | AppointmentStatus::Completed
            | AppointmentStatus::NoShow => {
                if cancelled_at.is_some() || cancellation_reason.is_some() {
                    return Err(AppointmentError::UnexpectedCancellationData);
                }
            }
        }

        Ok(())
    }

    fn validate_cancellation_time(
        created_at: AppointmentCreatedAt,
        cancelled_at: Option<AppointmentCancelledAt>
    ) -> Result<(), AppointmentError> {
        if let Some(cancelled_at) = cancelled_at {
            let created_at: NaiveDateTime = created_at.into();
            let cancelled_at: NaiveDateTime = cancelled_at.into();

            if cancelled_at < created_at {
                return Err(AppointmentError::InvalidCancellationTime);
            }
        }

        Ok(())
    }
}

// MARK: Behavior
impl Appointment {
    pub fn cancel(
        &mut self,
        cancelled_at: AppointmentCancelledAt,
        cancellation_reason: Option<AppointmentCancellationReason>
    ) -> Result<(), AppointmentError> {
        if self.status != AppointmentStatus::Scheduled {
            return Err(AppointmentError::InvalidStatusTransition);
        }

        Self::validate_cancellation_time(self.created_at, Some(cancelled_at))?;

        self.status = AppointmentStatus::Cancelled;
        self.cancelled_at = Some(cancelled_at);
        self.cancellation_reason = cancellation_reason;

        Ok(())
    }

    pub fn complete(&mut self) -> Result<(), AppointmentError> {
        if self.status != AppointmentStatus::Scheduled {
            return Err(AppointmentError::InvalidStatusTransition);
        }

        self.status = AppointmentStatus::Completed;

        Ok(())
    }

    pub fn mark_as_no_show(&mut self) -> Result<(), AppointmentError> {
        if self.status != AppointmentStatus::Scheduled {
            return Err(AppointmentError::InvalidStatusTransition);
        }

        self.status = AppointmentStatus::NoShow;

        Ok(())
    }
}

// MARK: Getters
impl Appointment {
    pub fn id(&self) -> AppointmentId {
        self.id
    }

    pub fn client_id(&self) -> UserId {
        self.client_id
    }

    pub fn salon_id(&self) -> SalonId {
        self.salon_id
    }

    pub fn specialist_id(&self) -> SpecialistId {
        self.specialist_id
    }

    pub fn status(&self) -> AppointmentStatus {
        self.status
    }
    
    pub fn starts_at(&self) -> AppointmentStartsAt {
        self.starts_at
    }

    pub fn ends_at(&self) -> AppointmentEndsAt {
        self.ends_at
    }

    pub fn total_price_snapshot(&self) -> &AppointmentTotalPriceSnapshot {
        &self.total_price_snapshot
    }

    pub fn total_duration_minutes_snapshot(&self) -> AppointmentTotalDurationMinutesSnapshot {
        self.total_duration_minutes_snapshot
    }

    pub fn cancellation_reason(&self) -> Option<&AppointmentCancellationReason> {
        self.cancellation_reason.as_ref()
    }
    
    pub fn created_at(&self) -> AppointmentCreatedAt {
        self.created_at
    }
    
    pub fn cancelled_at(&self) -> Option<AppointmentCancelledAt> {
        self.cancelled_at
    }

    pub fn is_scheduled(&self) -> bool {
        self.status == AppointmentStatus::Scheduled
    }
    
    pub fn is_cancelled(&self) -> bool {
        self.status == AppointmentStatus::Cancelled
    }
    
    pub fn is_completed(&self) -> bool {
        self.status == AppointmentStatus::Completed
    }
    
    pub fn is_no_show(&self) -> bool {
        self.status == AppointmentStatus::NoShow
    }
}
