#![cfg(feature = "bevy")]

use bevy::app::{App, Update};
use moria::runtime::{ReceiptNotification, ReceiptNotificationBridge, emit_terminal_notifications};

#[test]
fn public_bevy_notification_bridge_is_available_when_bevy_is_enabled() {
    let mut app = App::new();
    app.insert_resource(ReceiptNotificationBridge::try_new(1).unwrap())
        .add_message::<ReceiptNotification>()
        .add_systems(Update, emit_terminal_notifications);
    app.update();
}
