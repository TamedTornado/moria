#![cfg(feature = "bevy")]

use bevy::{
    app::{App, Update},
    ecs::{
        message::MessageReader,
        prelude::{ResMut, Resource},
        schedule::IntoScheduleConfigs,
    },
};
use moria::{
    canonical::{ReceiptId, WorldId},
    facade::{FrontierPosition, MoriaClient, ReplayStreamKey},
    runtime::{
        ReceiptNotification, ReceiptNotificationBridge, ReceiptState, emit_terminal_notifications,
    },
};

#[derive(Default, Resource)]
struct Notifications(Vec<ReceiptNotification>);

fn collect_notifications(
    mut reader: MessageReader<ReceiptNotification>,
    mut notifications: ResMut<Notifications>,
) {
    notifications.0.extend(reader.read().copied());
}

#[test]
fn external_genesis_workflow_returns_a_concrete_receipt_and_notifies() {
    let client = MoriaClient::try_new(1, 96).unwrap();
    let stream = ReplayStreamKey::try_from_bytes([1; 32]).unwrap();
    let receipt = client
        .begin_world(WorldId::from_bytes([2; 16]), stream)
        .publish_genesis()
        .unwrap();

    let ReceiptState::Ready(ready) = receipt.poll() else {
        panic!("public genesis admission must produce a terminal receipt")
    };
    assert_eq!(ready.frontier.position, FrontierPosition::Genesis);
    assert_eq!(ready.replay.stream, stream);

    let mut bridge = ReceiptNotificationBridge::try_new(1).unwrap();
    receipt.watch_terminal_notification(&mut bridge).unwrap();
    let mut app = App::new();
    app.insert_resource(bridge)
        .insert_resource(Notifications::default())
        .add_message::<ReceiptNotification>()
        .add_systems(
            Update,
            (emit_terminal_notifications, collect_notifications).chain(),
        );
    app.update();
    assert_eq!(
        app.world().resource::<Notifications>().0.as_slice(),
        &[ReceiptNotification {
            receipt: ReceiptId::from_raw(1),
            family: moria::runtime::ReceiptFamily::Genesis,
        }]
    );
}
