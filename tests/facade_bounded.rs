use moria::facade::{
    BoundedBytes, BoundedBytes64, BoundedOwnerError, BoundedPushRejected, BoundedUtf8, BoundedVec,
    OwnedBytes,
};
use std::sync::Arc;

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
fn zero_capacity_owners_accept_only_empty_values() {
    let mut values = BoundedVec::<u8>::try_with_capacity(0).unwrap();
    assert!(values.is_empty());
    assert_eq!(values.capacity(), 0);
    assert_eq!(
        values.try_push(1),
        Err(BoundedPushRejected {
            value: 1,
            reason: BoundedOwnerError::LengthExceedsCapacity,
        })
    );

    let mut bytes = BoundedBytes::try_with_capacity(0).unwrap();
    assert!(bytes.is_empty());
    assert_eq!(bytes.capacity(), 0);
    assert_eq!(
        bytes.try_extend_from_slice(&[1]),
        Err(BoundedOwnerError::LengthExceedsCapacity)
    );

    let empty = BoundedUtf8::<0>::try_from_bytes(Vec::new()).unwrap();
    assert!(empty.is_empty());
    assert_eq!(empty.as_str(), "");
    let rejected = BoundedUtf8::<0>::try_from_bytes(vec![b'a']).unwrap_err();
    assert_eq!(rejected.bytes, b"a");
    assert_eq!(rejected.reason, BoundedOwnerError::LengthExceedsCapacity);
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
    let exact = BoundedUtf8::<2>::try_from_bytes("é".as_bytes().to_vec()).unwrap();
    assert_eq!(exact.as_str(), "é");
    assert_eq!(exact.len(), 2);
    assert_eq!(exact.into_bytes(), "é".as_bytes());
}

#[test]
fn bounded_utf8_normalizes_spare_capacity_and_maps_allocation_failure() {
    let mut oversized_allocation = Vec::with_capacity(4_096);
    oversized_allocation.push(b'a');
    let text = BoundedUtf8::<1>::try_from_bytes(oversized_allocation).unwrap();
    let normalized = text.into_bytes();
    assert_eq!(normalized, b"a");
    assert!(normalized.capacity() <= 1);

    let rejected = BoundedUtf8::<{ usize::MAX }>::try_from_bytes(vec![b'a']).unwrap_err();
    assert_eq!(rejected.bytes, b"a");
    assert_eq!(rejected.reason, BoundedOwnerError::AllocationFailed);
}

#[test]
fn owned_bytes_are_immutable_share_clones_and_keep_the_exact_length() {
    let bytes = OwnedBytes::try_from_vec(vec![1, 2, 3], 3).unwrap();
    assert_eq!(bytes.len(), 3);
    assert!(!bytes.is_empty());
    assert_eq!(bytes.as_slice(), &[1, 2, 3]);

    let clone = bytes.clone();
    let into_arc: fn(OwnedBytes) -> Arc<[u8]> = OwnedBytes::into_arc;
    let original_arc = into_arc(bytes);
    let cloned_arc = into_arc(clone);
    assert_eq!(&*original_arc, &[1, 2, 3]);
    assert_eq!(&*cloned_arc, &[1, 2, 3]);
}
