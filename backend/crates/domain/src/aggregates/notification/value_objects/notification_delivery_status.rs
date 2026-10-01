#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NotificationDeliveryStatus {
    Pending,
    Processing,
    Delivered,
    Failed
}
