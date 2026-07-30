use moria::canonical::{
    BlobDigest, CanonicalHash, CanonicalOrder, ContentDigest, ContractDigest, DeviceGeneration,
    InputSourceId, MaterialId, NewtypeValueError, ParticipantId, ReceiptId, RngStreamId,
    SchemaDigest, Tick, VolumeId, VolumeRevision, WorldId,
};

#[test]
fn constrained_ids_reject_reserved_values_and_preserve_maximums() {
    assert_eq!(
        MaterialId::try_from_raw(0),
        Err(NewtypeValueError::ZeroReserved)
    );
    assert_eq!(
        VolumeId::try_from_raw(0),
        Err(NewtypeValueError::ZeroReserved)
    );
    assert_eq!(
        ParticipantId::try_from_raw(0),
        Err(NewtypeValueError::ZeroReserved)
    );
    assert_eq!(
        InputSourceId::try_from_raw(0),
        Err(NewtypeValueError::ZeroReserved)
    );
    assert_eq!(
        RngStreamId::try_from_raw(0),
        Err(NewtypeValueError::ZeroReserved)
    );

    assert_eq!(MaterialId::try_from_raw(u16::MAX).unwrap().get(), u16::MAX);
    assert_eq!(VolumeId::try_from_raw(u64::MAX).unwrap().get(), u64::MAX);
    assert_eq!(
        ParticipantId::try_from_raw(0x7fff_ffff).unwrap().get(),
        0x7fff_ffff
    );
    assert_eq!(
        InputSourceId::try_from_raw(0x7fff_ffff).unwrap().get(),
        0x7fff_ffff
    );
    assert_eq!(
        ParticipantId::try_from_raw(0x8000_0000),
        Err(NewtypeValueError::OutOfRange)
    );
    assert_eq!(
        InputSourceId::try_from_raw(u32::MAX),
        Err(NewtypeValueError::OutOfRange)
    );
}

#[test]
fn rng_stream_ids_accept_the_entire_nonzero_u32_domain() {
    for raw in [1, 0x7fff_ffff, 0x8000_0000, u32::MAX] {
        assert_eq!(RngStreamId::try_from_raw(raw).unwrap().get(), raw);
    }
}

#[test]
fn counter_newtypes_losslessly_represent_zero_and_exhaustion_boundary() {
    assert_eq!(Tick::from_raw(0).get(), 0);
    assert_eq!(VolumeRevision::from_raw(u64::MAX).get(), u64::MAX);
    assert_eq!(CanonicalOrder::from_raw(u32::MAX).get(), u32::MAX);
    assert_eq!(DeviceGeneration::from_raw(u64::MAX).get(), u64::MAX);
    assert_eq!(ReceiptId::from_raw(u64::MAX).get(), u64::MAX);
}

#[test]
fn world_and_digest_bytes_are_bit_preserving_and_type_distinct() {
    let world_bytes = [0xa5; 16];
    let world = WorldId::from_bytes(world_bytes);
    assert_eq!(world.as_bytes(), &world_bytes);
    assert_eq!(world.to_bytes(), world_bytes);

    let bytes = [0x5a; 32];
    assert_eq!(CanonicalHash::from_bytes(bytes).to_bytes(), bytes);
    assert_eq!(ContentDigest::from_bytes(bytes).to_bytes(), bytes);
    assert_eq!(ContractDigest::from_bytes(bytes).to_bytes(), bytes);
    assert_eq!(SchemaDigest::from_bytes(bytes).to_bytes(), bytes);
    assert_eq!(BlobDigest::from_bytes(bytes).to_bytes(), bytes);
}
