#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AppointmentStatus {
    Scheduled,
    Cancelled,
    Completed,
    NoShow
}
