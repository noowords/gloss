#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NotificationType {
    AppointmentCreated,
    AppointmentCancelled,
    AppointmentRescheduled,
    AppointmentReminder
}
