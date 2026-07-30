use moria::facade::{
    BoundedBytes, BoundedBytes64, BoundedOwnerError, BoundedUtf8, BoundedVec, OwnedBytes,
};

#[test]
fn bounded_vector_preserves_values_and_returns_rejected_value() {
    let mut values = BoundedVec::try_from_vec(vec![1, 2], 2).unwrap();

    assert_eq!(values.as_slice(), &[1, 2]);
    assert_eq!(values.iter().copied().collect::<Vec<_>>(), vec![1, 2]);
    assert_eq!(values.len(), 2);
    assert_eq!(values.capacity(), 2);
    assert_eq!(values.try_push(3).unwrap_err().value, 3);
    assert_eq!(values.into_vec(), vec![1, 2]);
}

#[test]
fn bounded_vector_returns_original_allocation_when_construction_exceeds_capacity() {
    let rejected = BoundedVec::try_from_vec(vec![1, 2], 1).unwrap_err();

    assert_eq!(rejected.reason, BoundedOwnerError::LengthExceedsCapacity);
    assert_eq!(rejected.values, vec![1, 2]);
}

#[test]
fn bounded_bytes_refuse_partial_extension() {
    let mut bytes = BoundedBytes::try_from_vec(vec![1, 2], 3).unwrap();

    assert_eq!(
        bytes.try_extend_from_slice(&[3, 4]),
        Err(BoundedOwnerError::LengthExceedsCapacity)
    );
    assert_eq!(bytes.as_slice(), &[1, 2]);
    bytes.try_extend_from_slice(&[3]).unwrap();
    assert_eq!(bytes.into_vec(), vec![1, 2, 3]);
}

#[test]
fn fixed_and_utf8_bytes_enforce_their_exact_bounds_without_losing_input() {
    assert!(BoundedBytes64::try_from_slice(&[]).unwrap().is_empty());
    assert_eq!(BoundedBytes64::try_from_slice(&[7; 64]).unwrap().len(), 64);
    assert_eq!(
        BoundedBytes64::try_from_slice(&[7; 65]),
        Err(BoundedOwnerError::LengthExceedsCapacity)
    );

    let utf8 = BoundedUtf8::<4>::try_from_bytes(b"rust".to_vec()).unwrap();
    assert_eq!(utf8.as_str(), "rust");
    assert_eq!(utf8.into_bytes(), b"rust");

    let invalid = BoundedUtf8::<4>::try_from_bytes(vec![0xff]).unwrap_err();
    assert_eq!(invalid.reason, BoundedOwnerError::InvalidUtf8);
    assert_eq!(invalid.bytes, vec![0xff]);
    let too_long = BoundedUtf8::<4>::try_from_bytes(b"rust!".to_vec()).unwrap_err();
    assert_eq!(too_long.reason, BoundedOwnerError::LengthExceedsCapacity);
    assert_eq!(too_long.bytes, b"rust!");
}

#[test]
fn bounded_utf8_normalizes_an_overallocated_input() {
    let mut input = Vec::with_capacity(16 * 1024);
    input.push(b'x');

    let output = BoundedUtf8::<1>::try_from_bytes(input)
        .unwrap()
        .into_bytes();

    assert_eq!(output, b"x");
    assert_eq!(output.capacity(), 1);
}

#[test]
fn owned_bytes_have_an_exact_immutable_length_and_recover_failed_input() {
    let owned = OwnedBytes::try_from_vec(vec![1, 2, 3], 3).unwrap();
    assert_eq!(owned.as_slice(), &[1, 2, 3]);
    assert_eq!(owned.len(), 3);
    assert!(!owned.is_empty());
    let shared = owned.clone();
    assert_eq!(shared.as_slice(), owned.as_slice());
    assert_eq!(shared.as_slice().as_ptr(), owned.as_slice().as_ptr());

    let rejected = OwnedBytes::try_from_vec(vec![1, 2, 3], 2).unwrap_err();
    assert_eq!(rejected.reason, BoundedOwnerError::LengthExceedsCapacity);
    assert_eq!(rejected.bytes, vec![1, 2, 3]);
}
