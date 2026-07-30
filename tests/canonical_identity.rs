use moria::canonical::{
    BlobDigest, CanonicalHash, CanonicalOrder, ContentDigest, ContractDigest, DeviceGeneration,
    InputSourceId, MaterialId, NewtypeValueError, ParticipantId, ReceiptId, RngStreamId,
    SchemaDigest, Tick, VolumeId, VolumeIdRegistry, VolumeRegistryError, VolumeRevision, WorldId,
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

#[test]
fn volume_registry_keeps_genesis_ids_unique_and_allocates_the_next_serial() {
    let low = VolumeId::try_from_raw(1).unwrap();
    let high = VolumeId::try_from_raw(41).unwrap();
    let mut registry = VolumeIdRegistry::from_genesis(3, &[low, high]).unwrap();

    assert_eq!(
        registry.next_volume_serial(),
        Some(VolumeId::try_from_raw(42).unwrap())
    );
    assert_eq!(
        registry.allocate_next(),
        Ok(VolumeId::try_from_raw(42).unwrap())
    );
    assert_eq!(
        registry.next_volume_serial(),
        Some(VolumeId::try_from_raw(43).unwrap())
    );
}

#[test]
fn volume_registry_reports_exhaustion_without_mutation() {
    let maximum = VolumeId::try_from_raw(u64::MAX).unwrap();
    let mut registry = VolumeIdRegistry::from_genesis(2, &[maximum]).unwrap();
    assert_eq!(registry.next_volume_serial(), None);
    let before = registry.clone();

    assert_eq!(
        registry.allocate_next(),
        Err(VolumeRegistryError::Exhausted)
    );
    assert_eq!(registry, before);

    let mut at_capacity =
        VolumeIdRegistry::from_genesis(1, &[VolumeId::try_from_raw(4).unwrap()]).unwrap();
    let before = at_capacity.clone();
    assert_eq!(
        at_capacity.allocate_next(),
        Err(VolumeRegistryError::Exhausted)
    );
    assert_eq!(at_capacity, before);
}

#[test]
fn invalid_genesis_registrations_do_not_create_a_registry() {
    let duplicate = VolumeId::try_from_raw(7).unwrap();
    assert_eq!(
        VolumeIdRegistry::from_genesis(2, &[duplicate, duplicate]),
        Err(VolumeRegistryError::DuplicateGenesisId(duplicate))
    );
    assert_eq!(
        VolumeIdRegistry::from_genesis(
            1,
            &[
                VolumeId::try_from_raw(1).unwrap(),
                VolumeId::try_from_raw(2).unwrap(),
            ],
        ),
        Err(VolumeRegistryError::Exhausted)
    );
}
