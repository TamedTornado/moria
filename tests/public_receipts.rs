use bevy::{app::App, ecs::message::Messages};
use moria::{
    canonical::{DeviceGeneration, ReceiptId},
    runtime::{
        CancelResult, InterestReceipt, ReceiptFamily, ReceiptNotification, ReceiptPolicy,
        ReceiptState, TerminalCache,
    },
};

#[test]
fn public_receipt_facade_polls_cancels_and_integrates_with_headless_bevy_messages() {
    let cache = TerminalCache::<u32, &'static str>::try_new(2, 16).unwrap();
    let receipt: InterestReceipt<u32, &'static str> = cache
        .admit(
            ReceiptId::from_raw(1),
            DeviceGeneration::from_raw(1),
            ReceiptPolicy::for_family(ReceiptFamily::Interest),
            8,
        )
        .unwrap()
        .into();

    assert!(matches!(receipt.poll(), ReceiptState::Pending(_)));
    assert_eq!(receipt.cancel(), CancelResult::CancelledBeforeSubmit);
    assert!(matches!(receipt.poll(), ReceiptState::Cancelled(_)));

    let generic = cache
        .admit(
            ReceiptId::from_raw(2),
            DeviceGeneration::from_raw(1),
            ReceiptPolicy::for_family(ReceiptFamily::Interest),
            8,
        )
        .unwrap();
    assert!(generic.terminal_notification().is_none());

    let mut app = App::new();
    app.add_message::<ReceiptNotification>();
    app.world_mut()
        .resource_mut::<Messages<ReceiptNotification>>()
        .write(receipt.terminal_notification().unwrap());
    app.update();
}
