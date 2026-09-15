use bigdecimal::{ BigDecimal };
use chrono::{ Duration, NaiveDate, NaiveDateTime, NaiveTime };

use domain::aggregates::{
    appointments::appointment::{ Appointment, value_objects::{ AppointmentId } },
    appointments::appointment::value_objects::AppointmentStatus,
    appointments::appointment_service::{ AppointmentService, value_objects::{ AppointmentServiceRole } },
    salons::salon::value_objects::{ SalonId, SalonTimezone },
    salons::salon_schedule_exception::value_objects::SalonScheduleExceptionType,
    salons::salon_service::{ SalonService, value_objects::{ SalonServicePrice } },
    services::service::{ Service, value_objects::{ ServiceKind, ServiceDurationMinutes } },
    services::service_addon_rule::{ ServiceAddonRule },
    specialists::specialist::{ Specialist },
    specialists::specialist_service::{ SpecialistService },
    specialists::specialist_time_off::{ SpecialistTimeOff },
    specialists::specialist_schedule::{ SpecialistSchedule },
    salons::salon_schedule_interval::{ SalonScheduleInterval, value_objects::{ SalonScheduleIntervalWeekday } },
    reviews::review::value_objects::{ ReviewRating },
    notifications::{
        notification::{ Notification },
        notification_delivery::{ NotificationDelivery, value_objects::{ NotificationDeliveryStatus } }
    },
    users::user::value_objects::{ UserId }
};

fn start() -> NaiveDateTime {
    NaiveDate::from_ymd_opt(2026, 9, 15).unwrap().and_hms_opt(10, 0, 0).unwrap()
}

fn service(kind: ServiceKind, duration: u16) -> Service {
    Service::create(
        "Manicure".try_into().unwrap(),
        None,
        None,
        "manicure".try_into().unwrap(),
        kind,
        duration.try_into().unwrap()
    ).unwrap()
}

struct Booking {
    specialist: Specialist,
    services: Vec<Service>,
    salon_services: Vec<SalonService>,
    specialist_services: Vec<SpecialistService>,
    rules: Vec<ServiceAddonRule>
}

impl Booking {
    fn new() -> Self {
        let specialist = Specialist::create(UserId::generate(), SalonId::generate(), None, None).unwrap();
        let services = vec![service(ServiceKind::Primary, 60), service(ServiceKind::Addon, 15)];
        let salon_services = services.iter().map(|service| {
            SalonService::create(specialist.salon_id(), service.id(), "100.25".try_into().unwrap()).unwrap()
        }).collect();
        let specialist_services = services.iter().map(|service| {
            SpecialistService::create(specialist.id(), specialist.salon_id(), service.id()).unwrap()
        }).collect();
        let rules = vec![ServiceAddonRule::create(&services[0], &services[1]).unwrap()];

        Self { specialist, services, salon_services, specialist_services, rules }
    }

    fn create(&self) -> Result<Appointment, anyhow::Error> {
        Appointment::create(
            UserId::generate(),
            &self.specialist,
            start().into(),
            &self.services,
            &self.salon_services,
            &self.specialist_services,
            &self.rules
        )
    }
}

#[test]
fn booking_captures_prices_and_duration() {
    let mut booking = Booking::new();
    let appointment = booking.create().unwrap();
    booking.salon_services.clear();

    assert_eq!(BigDecimal::from(appointment.total_price_snapshot()), "200.50".parse::<BigDecimal>().unwrap());
    assert_eq!(u16::from(appointment.total_duration_minutes_snapshot()), 75);
    assert_eq!(NaiveDateTime::from(appointment.ends_at()), start() + Duration::minutes(75));
    assert_eq!(String::from(appointment.status()), "scheduled");
    assert_eq!(appointment.services().len(), 2);
    assert!(appointment.services().iter().all(|service| service.appointment_id() == appointment.id()));
}

#[test]
fn booking_rejects_invalid_service_selections() {
    let mut booking = Booking::new();
    booking.services.clear();
    assert!(booking.create().is_err());

    let mut booking = Booking::new();
    booking.services.remove(0);
    assert!(booking.create().is_err());

    let mut booking = Booking::new();
    booking.services.push(booking.services[0].clone());
    assert!(booking.create().is_err());

    let mut booking = Booking::new();
    booking.services.push(booking.services[1].clone());
    assert!(booking.create().is_err());

    let mut booking = Booking::new();
    booking.rules.clear();
    assert!(booking.create().is_err());

    let mut booking = Booking::new();
    booking.salon_services.pop();
    assert!(booking.create().is_err());

    let mut booking = Booking::new();
    booking.specialist_services.pop();
    assert!(booking.create().is_err());
}

#[test]
fn booking_rejects_cross_salon_assignments() {
    let mut booking = Booking::new();
    booking.specialist_services[1] = SpecialistService::create(
        booking.specialist.id(), SalonId::generate(), booking.services[1].id()
    ).unwrap();
    assert!(booking.create().is_err());

    let mut booking = Booking::new();
    booking.salon_services[1] = SalonService::create(
        SalonId::generate(), booking.services[1].id(), "100".try_into().unwrap()
    ).unwrap();
    assert!(booking.create().is_err());
}

#[test]
fn addon_rule_requires_correct_kinds() {
    let primary = service(ServiceKind::Primary, 60);
    let addon = service(ServiceKind::Addon, 15);
    assert!(ServiceAddonRule::create(&addon, &primary).is_err());
    assert!(ServiceAddonRule::create(&primary, &primary).is_err());
    assert!(ServiceAddonRule::restore(primary.id(), primary.id()).is_err());
}

#[test]
fn restore_rejects_inconsistent_snapshots() {
    let appointment = Booking::new().create().unwrap();
    let restore = |price, services| Appointment::restore(
        appointment.id(), appointment.client_id(), appointment.salon_id(), appointment.specialist_id(),
        appointment.starts_at(), appointment.ends_at(), appointment.status(), price,
        appointment.total_duration_minutes_snapshot(), None, None, services
    );
    assert!(restore(appointment.total_price_snapshot(), appointment.services().to_vec()).is_ok());
    assert!(restore("999".try_into().unwrap(), appointment.services().to_vec()).is_err());
    let mut snapshots = appointment.services().to_vec();
    snapshots[0] = AppointmentService::create(
        AppointmentId::generate(), snapshots[0].service_id(), AppointmentServiceRole::Primary,
        snapshots[0].service_name_snapshot(), snapshots[0].price_snapshot(), snapshots[0].duration_minutes_snapshot()
    ).unwrap();
    assert!(restore(appointment.total_price_snapshot(), snapshots).is_err());
}

#[test]
fn numeric_values_follow_database_bounds() {
    assert!(ServiceDurationMinutes::try_from(0u16).is_err());
    assert!(ServiceDurationMinutes::try_from(u16::MAX).is_ok());
    for price in ["-1", "100000000", "1.001"] {
        assert!(SalonServicePrice::try_from(price).is_err());
    }
    for price in ["0", "99999999.99", "1.230"] {
        assert!(SalonServicePrice::try_from(price).is_ok());
    }
    assert!(ReviewRating::try_from(0u8).is_err());
    assert!(ReviewRating::try_from(6u8).is_err());
    assert!(ReviewRating::try_from(5u8).is_ok());
    assert!(SalonScheduleIntervalWeekday::try_from(0u8).is_err());
    assert!(SalonScheduleIntervalWeekday::try_from(8u8).is_err());
}

#[test]
fn timezone_requires_an_iana_name() {
    assert!(SalonTimezone::try_from("Asia/Yakutsk").is_ok());
    assert!(SalonTimezone::try_from("UTC").is_ok());
    assert!(SalonTimezone::try_from("Unknown/Timezone").is_err());
    assert!(SalonTimezone::try_from("").is_err());
}

#[test]
fn booking_rejects_total_overflow() {
    let mut booking = Booking::new();
    for index in 0..booking.services.len() {
        booking.salon_services[index] = SalonService::create(
            booking.specialist.salon_id(), booking.services[index].id(), "99999999.99".try_into().unwrap()
        ).unwrap();
    }
    assert!(booking.create().is_err());

    let mut booking = Booking::new();
    let primary = &booking.services[0];
    booking.services[0] = Service::restore(
        primary.id(), primary.name(), primary.description(), primary.preview_url(), primary.category(),
        primary.kind(), u16::MAX.try_into().unwrap(), primary.is_active()
    ).unwrap();
    assert!(booking.create().is_err());
}

#[test]
fn schedules_reject_invalid_intervals() {
    let specialist = Booking::new().specialist;
    let time = NaiveTime::from_hms_opt(10, 0, 0).unwrap();
    assert!(SalonScheduleInterval::create(
        specialist.salon_id(), 1u8.try_into().unwrap(), time.into(), time.into()
    ).is_err());
    assert!(SpecialistSchedule::create(
        specialist.id(), start().date().into(), Some((start().date() - Duration::days(1)).into())
    ).is_err());
    assert!(SpecialistTimeOff::create(
        specialist.id(), start().date().into(), "leave".try_into().unwrap(), Some(time.into()), None, None
    ).is_err());
    assert!(SpecialistTimeOff::create(
        specialist.id(), start().date().into(), "leave".try_into().unwrap(), None, None, None
    ).is_ok());
}

#[test]
fn notification_delivery_starts_pending() {
    let notification = Notification::create(
        UserId::generate(),
        "appointment".try_into().unwrap(),
        "Appointment".try_into().unwrap(),
        "Your appointment is scheduled".try_into().unwrap()
    ).unwrap();
    let delivery = NotificationDelivery::create(notification.id(), "telegram".try_into().unwrap()).unwrap();

    assert_eq!(delivery.notification_id(), notification.id());
    assert_eq!(delivery.status(), NotificationDeliveryStatus::Pending);
    assert!(delivery.attempted_at().is_none());
    assert!(delivery.delivered_at().is_none());
    assert!(delivery.read_at().is_none());
    assert!(delivery.error().is_none());
}

#[test]
fn schema_statuses_are_restricted() {
    assert!(AppointmentStatus::try_from("no_show").is_ok());
    assert!(AppointmentStatus::try_from("waiting").is_err());
    assert!(SalonScheduleExceptionType::try_from("custom_hours").is_ok());
    assert!(SalonScheduleExceptionType::try_from("holiday").is_err());
    assert!(NotificationDeliveryStatus::try_from("processing").is_ok());
    assert!(NotificationDeliveryStatus::try_from("retrying").is_err());
}
