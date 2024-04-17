use std::sync::RwLock;

use lazy_static::lazy_static;

use crate::model::notification::Notification;

// Singleton of Database
lazy_static! {
    pub static ref NOTIFICATIONS: RwLock<Vec<Notification>> = RwLock::new(vec![]);
}

pub struct NotificationRepository;
impl NotificationRepository {
    pub fn add(notification: Notification) -> Notification {
        NOTIFICATIONS.write().unwrap()
            .push(notification.clone());
        return notification;
    }
}