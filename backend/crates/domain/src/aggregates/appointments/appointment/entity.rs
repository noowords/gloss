use bigdecimal::{ BigDecimal, Zero };
use chrono::{ Duration, NaiveDateTime };

use crate::aggregates::{
    services::service::{ Service, value_objects::{ ServiceKind } },
    salons::salon_service::{ SalonService },
    specialists::specialist::{ Specialist },
    specialists::specialist_service::{ SpecialistService },
    services::service_addon_rule::{ ServiceAddonRule },
    appointments::appointment_service::{ AppointmentService, value_objects::{ AppointmentServiceRole } }
};

use crate::aggregates::users::user::value_objects::{ UserId };
use crate::aggregates::salons::salon::value_objects::{ SalonId };
use crate::aggregates::specialists::specialist::value_objects::{ SpecialistId };

use super::value_objects::{ AppointmentId, AppointmentStartsAt, AppointmentEndsAt, AppointmentStatus, AppointmentTotalPriceSnapshot, AppointmentTotalDurationMinutesSnapshot, AppointmentCancelledAt, AppointmentCancellationReason };

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Appointment {
    id: AppointmentId,
    client_id: UserId,
    salon_id: SalonId,
    specialist_id: SpecialistId,
    starts_at: AppointmentStartsAt,
    ends_at: AppointmentEndsAt,
    status: AppointmentStatus,
    total_price_snapshot: AppointmentTotalPriceSnapshot,
    total_duration_minutes_snapshot: AppointmentTotalDurationMinutesSnapshot,
    cancelled_at: Option<AppointmentCancelledAt>,
    cancellation_reason: Option<AppointmentCancellationReason>,
    services: Vec<AppointmentService>
}

impl Appointment {
    pub fn create(
        client_id: UserId,
        specialist: &Specialist,
        starts_at: AppointmentStartsAt,
        services: &[Service],
        salon_services: &[SalonService],
        specialist_services: &[SpecialistService],
        addon_rules: &[ServiceAddonRule]
    ) -> Result<Self, anyhow::Error> {
        if String::from(specialist.status()) != "active" {
            anyhow::bail!("The specialist must be active");
        }

        let primary = services.iter().filter(|service| service.kind() == ServiceKind::Primary).collect::<Vec<_>>();

        if primary.len() != 1 {
            anyhow::bail!("An appointment requires exactly one primary service");
        }

        let id = AppointmentId::generate();
        let mut snapshots = Vec::new();
        let mut total_price = BigDecimal::zero();
        let mut total_duration = 0u16;

        for service in services {
            if !bool::from(service.is_active()) {
                anyhow::bail!("The service must be active");
            }

            if snapshots.iter().any(|snapshot: &AppointmentService| snapshot.service_id() == service.id()) {
                anyhow::bail!("An appointment cannot contain duplicate services");
            }

            if service.kind() == ServiceKind::Addon && !addon_rules.iter().any(|rule| {
                rule.primary_service_id() == primary[0].id() && rule.addon_service_id() == service.id()
            }) {
                anyhow::bail!("The addon is not allowed for the primary service");
            }

            let salon_service = salon_services.iter().find(|salon_service| {
                salon_service.salon_id() == specialist.salon_id()
                    && salon_service.service_id() == service.id()
                    && bool::from(salon_service.is_active())
            }).ok_or_else(|| anyhow::anyhow!("The service is not available in the salon"))?;

            if !specialist_services.iter().any(|specialist_service| {
                specialist_service.specialist_id() == specialist.id()
                    && specialist_service.salon_id() == specialist.salon_id()
                    && specialist_service.service_id() == service.id()
            }) {
                anyhow::bail!("The specialist does not provide the service in this salon");
            }

            let price = BigDecimal::from(salon_service.price());
            let duration = u16::from(service.duration_minutes());
            total_price += &price;
            total_duration = total_duration.checked_add(duration)
                .ok_or_else(|| anyhow::anyhow!("The total appointment duration is too large"))?;

            snapshots.push(AppointmentService::create(
                id,
                service.id(),
                match service.kind() {
                    ServiceKind::Primary => AppointmentServiceRole::Primary,
                    ServiceKind::Addon => AppointmentServiceRole::Addon
                },
                String::from(service.name()).try_into()?,
                price.try_into()?,
                duration.try_into()?
            )?);
        }

        let ends_at = NaiveDateTime::from(starts_at)
            .checked_add_signed(Duration::minutes(i64::from(total_duration)))
            .ok_or_else(|| anyhow::anyhow!("The appointment end is out of range"))?;

        Self::restore(
            id,
            client_id,
            specialist.salon_id(),
            specialist.id(),
            starts_at,
            ends_at.into(),
            "scheduled".try_into()?,
            total_price.try_into()?,
            total_duration.try_into()?,
            None,
            None,
            snapshots
        )
    }

    pub fn restore(
        id: AppointmentId,
        client_id: UserId,
        salon_id: SalonId,
        specialist_id: SpecialistId,
        starts_at: AppointmentStartsAt,
        ends_at: AppointmentEndsAt,
        status: AppointmentStatus,
        total_price_snapshot: AppointmentTotalPriceSnapshot,
        total_duration_minutes_snapshot: AppointmentTotalDurationMinutesSnapshot,
        cancelled_at: Option<AppointmentCancelledAt>,
        cancellation_reason: Option<AppointmentCancellationReason>,
        services: Vec<AppointmentService>
    ) -> Result<Self, anyhow::Error> {
        if chrono::NaiveDateTime::from(starts_at) >= chrono::NaiveDateTime::from(ends_at) {
            anyhow::bail!("The interval must end after it starts");
        }

        if services.iter().filter(|service| service.role() == AppointmentServiceRole::Primary).count() != 1 {
            anyhow::bail!("An appointment requires exactly one primary service");
        }

        let mut price = BigDecimal::zero();
        let mut duration = 0u16;

        for (index, service) in services.iter().enumerate() {
            if service.appointment_id() != id {
                anyhow::bail!("The service snapshot belongs to another appointment");
            }

            if services[..index].iter().any(|previous| previous.service_id() == service.service_id()) {
                anyhow::bail!("An appointment cannot contain duplicate services");
            }

            price += BigDecimal::from(service.price_snapshot());
            duration = duration.checked_add(u16::from(service.duration_minutes_snapshot()))
                .ok_or_else(|| anyhow::anyhow!("The total appointment duration is too large"))?;
        }

        if price != BigDecimal::from(total_price_snapshot.clone()) || duration != u16::from(total_duration_minutes_snapshot) {
            anyhow::bail!("Appointment totals must match its service snapshots");
        }

        if NaiveDateTime::from(ends_at).signed_duration_since(NaiveDateTime::from(starts_at)) != Duration::minutes(i64::from(duration)) {
            anyhow::bail!("The appointment interval must match its total duration");
        }

        Ok(Self {
            id,
            client_id,
            salon_id,
            specialist_id,
            starts_at,
            ends_at,
            status,
            total_price_snapshot,
            total_duration_minutes_snapshot,
            cancelled_at,
            cancellation_reason,
            services
        })
    }

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

    pub fn starts_at(&self) -> AppointmentStartsAt {
        self.starts_at
    }

    pub fn ends_at(&self) -> AppointmentEndsAt {
        self.ends_at
    }

    pub fn status(&self) -> AppointmentStatus {
        self.status.clone()
    }

    pub fn total_price_snapshot(&self) -> AppointmentTotalPriceSnapshot {
        self.total_price_snapshot.clone()
    }

    pub fn total_duration_minutes_snapshot(&self) -> AppointmentTotalDurationMinutesSnapshot {
        self.total_duration_minutes_snapshot
    }

    pub fn cancelled_at(&self) -> Option<AppointmentCancelledAt> {
        self.cancelled_at
    }

    pub fn cancellation_reason(&self) -> Option<AppointmentCancellationReason> {
        self.cancellation_reason.clone()
    }

    pub fn services(&self) -> &[AppointmentService] {
        &self.services
    }
}
