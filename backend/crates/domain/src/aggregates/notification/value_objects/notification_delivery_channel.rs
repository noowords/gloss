#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum NotificationDeliveryChannel {
    InApp,
    Sms,
    Telegram
}
