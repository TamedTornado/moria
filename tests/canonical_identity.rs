use moria::canonical::{
    BlobDigest, CanonicalHash, CanonicalOrder, ContentDigest, ContractDigest, DeviceGeneration,
    InputSourceId, MaterialId, NewtypeValueError, ParticipantId, ReceiptId, RngStreamId,
    SchemaDigest, Tick, VolumeId, VolumeRevision, WorldId,
};

#[test]
fn constrained_ids_cover_zero_one_maximum_and_first_invalid_values() {
    assert_eq!(
        MaterialId::try_from_raw(0),
        Err(NewtypeValueError::ZeroReserved)
    );
    assert_eq!(MaterialId::try_from_raw(1).unwrap().get(), 1);
    assert_eq!(MaterialId::try_from_raw(u16::MAX).unwrap().get(), u16::MAX);

    assert_eq!(
        VolumeId::try_from_raw(0),
        Err(NewtypeValueError::ZeroReserved)
    );
    assert_eq!(VolumeId::try_from_raw(1).unwrap().get(), 1);
    assert_eq!(VolumeId::try_from_raw(u64::MAX).unwrap().get(), u64::MAX);

    assert_eq!(
        ParticipantId::try_from_raw(0),
        Err(NewtypeValueError::ZeroReserved)
    );
    assert_eq!(ParticipantId::try_from_raw(1).unwrap().get(), 1);
    assert_eq!(
        ParticipantId::try_from_raw(0x7fff_ffff).unwrap().get(),
        0x7fff_ffff
    );
    assert_eq!(
        ParticipantId::try_from_raw(0x8000_0000),
        Err(NewtypeValueError::OutOfRange)
    );

    assert_eq!(
        InputSourceId::try_from_raw(0),
        Err(NewtypeValueError::ZeroReserved)
    );
    assert_eq!(InputSourceId::try_from_raw(1).unwrap().get(), 1);
    assert_eq!(
        InputSourceId::try_from_raw(0x7fff_ffff).unwrap().get(),
        0x7fff_ffff
    );
    assert_eq!(
        InputSourceId::try_from_raw(0x8000_0000),
        Err(NewtypeValueError::OutOfRange)
    );

    assert_eq!(
        RngStreamId::try_from_raw(0),
        Err(NewtypeValueError::ZeroReserved)
    );
    for raw in [1, 0x7fff_ffff, 0x8000_0000, u32::MAX] {
        assert_eq!(RngStreamId::try_from_raw(raw).unwrap().get(), raw);
    }
}

#[test]
fn every_counter_round_trips_zero_and_its_extreme() {
    for value in [0, u64::MAX] {
        assert_eq!(Tick::from_raw(value).get(), value);
        assert_eq!(VolumeRevision::from_raw(value).get(), value);
        assert_eq!(DeviceGeneration::from_raw(value).get(), value);
        assert_eq!(ReceiptId::from_raw(value).get(), value);
    }
    for value in [0, u32::MAX] {
        assert_eq!(CanonicalOrder::from_raw(value).get(), value);
    }
}

#[test]
fn world_and_every_digest_preserve_required_byte_patterns() {
    let alternating_world = core::array::from_fn(|index| if index % 2 == 0 { 0xaa } else { 0x55 });
    for world_bytes in [[0; 16], [u8::MAX; 16], alternating_world, [0x80; 16]] {
        let world = WorldId::from_bytes(world_bytes);
        assert_eq!(world.as_bytes(), &world_bytes);
        assert_eq!(world.to_bytes(), world_bytes);
    }

    let alternating_digest = core::array::from_fn(|index| if index % 2 == 0 { 0xaa } else { 0x55 });
    for bytes in [[0; 32], [u8::MAX; 32], alternating_digest, [0x80; 32]] {
        assert_eq!(CanonicalHash::from_bytes(bytes).as_bytes(), &bytes);
        assert_eq!(CanonicalHash::from_bytes(bytes).to_bytes(), bytes);
        assert_eq!(ContentDigest::from_bytes(bytes).as_bytes(), &bytes);
        assert_eq!(ContentDigest::from_bytes(bytes).to_bytes(), bytes);
        assert_eq!(ContractDigest::from_bytes(bytes).as_bytes(), &bytes);
        assert_eq!(ContractDigest::from_bytes(bytes).to_bytes(), bytes);
        assert_eq!(SchemaDigest::from_bytes(bytes).as_bytes(), &bytes);
        assert_eq!(SchemaDigest::from_bytes(bytes).to_bytes(), bytes);
        assert_eq!(BlobDigest::from_bytes(bytes).as_bytes(), &bytes);
        assert_eq!(BlobDigest::from_bytes(bytes).to_bytes(), bytes);
    }
}
