use moria::facade::{
    BoundedBytes, BoundedBytes64, BoundedOwnerError, BoundedPushRejected, BoundedUtf8, BoundedVec,
    OwnedBytes,
};

#[test]
fn bounded_vec_preserves_exact_capacity_and_rejected_value() {
    let mut values = BoundedVec::try_with_capacity(2).unwrap();
    assert_eq!(values.len(), 0);
    assert_eq!(values.capacity(), 2);
    assert!(values.is_empty());

    values.try_push(3).unwrap();
    values.try_push(5).unwrap();
    assert_eq!(values.as_slice(), &[3, 5]);
    assert_eq!(values.iter().copied().collect::<Vec<_>>(), vec![3, 5]);

    assert_eq!(
        values.try_push(8),
        Err(BoundedPushRejected {
            value: 8,
            reason: BoundedOwnerError::LengthExceedsCapacity,
        })
    );
    assert_eq!(values.as_slice(), &[3, 5]);
    assert_eq!(values.into_vec(), vec![3, 5]);
}

#[test]
fn bounded_construction_failures_return_the_original_allocations() {
    let vector = BoundedVec::try_from_vec(vec![1, 2], 1).unwrap_err();
    assert_eq!(vector.values, vec![1, 2]);
    assert_eq!(vector.reason, BoundedOwnerError::LengthExceedsCapacity);

    let bytes = BoundedBytes::try_from_vec(vec![1, 2], 1).unwrap_err();
    assert_eq!(bytes.bytes, vec![1, 2]);
    assert_eq!(bytes.reason, BoundedOwnerError::LengthExceedsCapacity);

    let owned = OwnedBytes::try_from_vec(vec![1, 2], 1).unwrap_err();
    assert_eq!(owned.bytes, vec![1, 2]);
    assert_eq!(owned.reason, BoundedOwnerError::LengthExceedsCapacity);
}

#[test]
fn bounded_bytes_does_not_partially_extend() {
    let mut bytes = BoundedBytes::try_from_vec(vec![1, 2], 3).unwrap();
    assert_eq!(bytes.capacity(), 3);
    assert_eq!(
        bytes.try_extend_from_slice(&[3, 4]),
        Err(BoundedOwnerError::LengthExceedsCapacity)
    );
    assert_eq!(bytes.as_slice(), &[1, 2]);

    bytes.try_extend_from_slice(&[3]).unwrap();
    assert_eq!(bytes.len(), 3);
    assert_eq!(bytes.into_vec(), vec![1, 2, 3]);
}

#[test]
fn fixed_and_utf8_owners_validate_and_preserve_bytes() {
    assert_eq!(
        BoundedBytes64::try_from_slice(&[9; 65]),
        Err(BoundedOwnerError::LengthExceedsCapacity)
    );
    let fixed = BoundedBytes64::try_from_slice(&[9; 64]).unwrap();
    assert_eq!(fixed.len(), 64);
    assert_eq!(fixed.as_slice(), &[9; 64]);

    let invalid = BoundedUtf8::<3>::try_from_bytes(vec![0xff]).unwrap_err();
    assert_eq!(invalid.bytes, vec![0xff]);
    assert_eq!(invalid.reason, BoundedOwnerError::InvalidUtf8);
    let oversized = BoundedUtf8::<3>::try_from_bytes(vec![b'a', b'b', b'c', b'd']).unwrap_err();
    assert_eq!(oversized.bytes, b"abcd");
    assert_eq!(oversized.reason, BoundedOwnerError::LengthExceedsCapacity);
    let text = BoundedUtf8::<3>::try_from_bytes("é".as_bytes().to_vec()).unwrap();
    assert_eq!(text.as_str(), "é");
    assert_eq!(text.len(), 2);
    assert_eq!(text.into_bytes(), "é".as_bytes());
}

#[test]
fn owned_bytes_are_immutable_and_keep_the_exact_length() {
    let bytes = OwnedBytes::try_from_vec(vec![1, 2, 3], 3).unwrap();
    assert_eq!(bytes.len(), 3);
    assert!(!bytes.is_empty());
    assert_eq!(bytes.as_slice(), &[1, 2, 3]);
    assert_eq!(&*bytes.into_arc(), &[1, 2, 3]);
}
