#[cfg(test)]
mod tests {
    use super::{
        BoundedBytes, BoundedBytes64, BoundedOwnerError, BoundedUtf8, BoundedVec, OwnedBytes,
    };

    #[test]
    fn bounded_vec_preserves_capacity_and_rejected_value() {
        let mut values = BoundedVec::try_with_capacity(1).unwrap();
        values.try_push("first").unwrap();

        let rejected = values.try_push("second").unwrap_err();
        assert_eq!(rejected.value, "second");
        assert_eq!(rejected.reason, BoundedOwnerError::LengthExceedsCapacity);
        assert_eq!(values.capacity(), 1);
        assert_eq!(values.as_slice(), ["first"]);
    }

    #[test]
    fn vector_and_byte_construction_return_original_values() {
        let values = vec![1_u8, 2];
        let rejected = BoundedVec::try_from_vec(values, 1).unwrap_err();
        assert_eq!(rejected.reason, BoundedOwnerError::LengthExceedsCapacity);
        assert_eq!(rejected.values, vec![1, 2]);

        let bytes = vec![3_u8, 4];
        let rejected = BoundedBytes::try_from_vec(bytes, 1).unwrap_err();
        assert_eq!(rejected.reason, BoundedOwnerError::LengthExceedsCapacity);
        assert_eq!(rejected.bytes, vec![3, 4]);
    }

    #[test]
    fn byte_extension_is_all_or_nothing() {
        let mut bytes = BoundedBytes::try_from_vec(vec![1, 2], 3).unwrap();
        assert_eq!(
            bytes.try_extend_from_slice(&[3, 4]),
            Err(BoundedOwnerError::LengthExceedsCapacity)
        );
        assert_eq!(bytes.as_slice(), [1, 2]);
        assert_eq!(bytes.capacity(), 3);
    }

    #[test]
    fn fixed_and_utf8_owners_validate_their_lengths_and_values() {
        assert!(matches!(
            BoundedBytes64::try_from_slice(&[0; 65]),
            Err(BoundedOwnerError::LengthExceedsCapacity)
        ));
        let fixed = BoundedBytes64::try_from_slice(&[7, 8]).unwrap();
        assert_eq!(fixed.len(), 2);
        assert_eq!(fixed.as_slice(), [7, 8]);

        let invalid = BoundedUtf8::<8>::try_from_bytes(vec![0xff]).unwrap_err();
        assert_eq!(invalid.reason, BoundedOwnerError::InvalidUtf8);
        assert_eq!(invalid.bytes, vec![0xff]);

        let oversized = BoundedUtf8::<2>::try_from_bytes(b"abc".to_vec()).unwrap_err();
        assert_eq!(oversized.reason, BoundedOwnerError::LengthExceedsCapacity);
        assert_eq!(oversized.bytes, b"abc");
    }

    #[test]
    fn immutable_bytes_preserve_exact_content() {
        let bytes = OwnedBytes::try_from_vec(vec![9, 10], 2).unwrap();
        assert_eq!(bytes.len(), 2);
        assert_eq!(bytes.as_slice(), [9, 10]);
        assert_eq!(bytes.into_arc().as_ref(), [9, 10]);

        let rejected = OwnedBytes::try_from_vec(vec![9, 10], 1).unwrap_err();
        assert_eq!(rejected.reason, BoundedOwnerError::LengthExceedsCapacity);
        assert_eq!(rejected.bytes, vec![9, 10]);
    }

    #[test]
    fn allocation_failure_is_reported_without_accepting_an_owner() {
        assert!(matches!(
            super::try_allocate_with::<u8>(1, |_, _| Err(())),
            Err(BoundedOwnerError::AllocationFailed)
        ));
    }
}
use std::{mem::size_of, sync::Arc};

/// The reason construction or bounded growth was rejected.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundedOwnerError {
    /// The requested capacity cannot be represented by a finite allocation.
    CapacityTooLarge,
    /// The supplied length would exceed the owner's declared capacity.
    LengthExceedsCapacity,
    /// The supplied byte sequence is not valid UTF-8.
    InvalidUtf8,
    /// The allocator could not reserve the required finite storage.
    AllocationFailed,
}

/// A rejected vector construction that returns its input allocation unchanged.
#[derive(Debug, Eq, PartialEq)]
pub struct VecConstructionRejected<T> {
    /// The original values supplied to construction.
    pub values: Vec<T>,
    /// Why construction was rejected.
    pub reason: BoundedOwnerError,
}

/// A rejected byte construction that returns its input allocation unchanged.
#[derive(Debug, Eq, PartialEq)]
pub struct BytesConstructionRejected {
    /// The original bytes supplied to construction.
    pub bytes: Vec<u8>,
    /// Why construction was rejected.
    pub reason: BoundedOwnerError,
}

/// A rejected bounded-vector extension that returns the uninserted value.
#[derive(Debug, Eq, PartialEq)]
pub struct BoundedPushRejected<T> {
    /// The value that was not inserted.
    pub value: T,
    /// Why insertion was rejected.
    pub reason: BoundedOwnerError,
}

/// A finite vector whose logical capacity is immutable after construction.
#[derive(Debug)]
pub struct BoundedVec<T> {
    values: Vec<T>,
    capacity: u32,
}

impl<T> BoundedVec<T> {
    /// Allocates an empty owner with exactly `capacity` permitted values.
    pub fn try_with_capacity(capacity: u32) -> Result<Self, BoundedOwnerError> {
        Ok(Self {
            values: try_allocate(capacity)?,
            capacity,
        })
    }

    /// Takes `values` only when their count fits the declared capacity.
    pub fn try_from_vec(values: Vec<T>, capacity: u32) -> Result<Self, VecConstructionRejected<T>> {
        if !fits_capacity(values.len(), capacity) {
            return Err(VecConstructionRejected {
                values,
                reason: BoundedOwnerError::LengthExceedsCapacity,
            });
        }

        let mut bounded = match Self::try_with_capacity(capacity) {
            Ok(bounded) => bounded,
            Err(reason) => return Err(VecConstructionRejected { values, reason }),
        };
        bounded.values.extend(values);
        Ok(bounded)
    }

    /// Inserts one value, or returns that value untouched when the owner is full.
    pub fn try_push(&mut self, value: T) -> Result<(), BoundedPushRejected<T>> {
        if self.len() == self.capacity {
            return Err(BoundedPushRejected {
                value,
                reason: BoundedOwnerError::LengthExceedsCapacity,
            });
        }
        self.values.push(value);
        Ok(())
    }

    /// Returns the stored values without exposing the allocation for mutation.
    pub fn as_slice(&self) -> &[T] {
        &self.values
    }

    /// Iterates over the stored values in their insertion order.
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.values.iter()
    }

    /// Returns the exact number of stored values.
    pub fn len(&self) -> u32 {
        u32::try_from(self.values.len()).expect("bounded vector length fits its u32 capacity")
    }

    /// Returns the immutable logical capacity selected at construction.
    pub fn capacity(&self) -> u32 {
        self.capacity
    }

    /// Reports whether the owner contains no values.
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Transfers the owned values to the caller.
    pub fn into_vec(self) -> Vec<T> {
        self.values
    }
}

/// A finite byte owner whose logical capacity is immutable after construction.
#[derive(Debug)]
pub struct BoundedBytes {
    bytes: BoundedVec<u8>,
}

impl BoundedBytes {
    /// Allocates an empty byte owner with exactly `capacity` permitted bytes.
    pub fn try_with_capacity(capacity: u32) -> Result<Self, BoundedOwnerError> {
        BoundedVec::try_with_capacity(capacity).map(|bytes| Self { bytes })
    }

    /// Takes `bytes` only when their length fits the declared capacity.
    pub fn try_from_vec(bytes: Vec<u8>, capacity: u32) -> Result<Self, BytesConstructionRejected> {
        BoundedVec::try_from_vec(bytes, capacity)
            .map(|bytes| Self { bytes })
            .map_err(|rejected| BytesConstructionRejected {
                bytes: rejected.values,
                reason: rejected.reason,
            })
    }

    /// Appends all bytes or leaves this owner unchanged when they will not fit.
    pub fn try_extend_from_slice(&mut self, bytes: &[u8]) -> Result<(), BoundedOwnerError> {
        let new_len = self
            .bytes
            .values
            .len()
            .checked_add(bytes.len())
            .ok_or(BoundedOwnerError::LengthExceedsCapacity)?;
        if !fits_capacity(new_len, self.capacity()) {
            return Err(BoundedOwnerError::LengthExceedsCapacity);
        }
        self.bytes.values.extend_from_slice(bytes);
        Ok(())
    }

    /// Returns the stored bytes without exposing the allocation for mutation.
    pub fn as_slice(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    /// Returns the exact number of stored bytes.
    pub fn len(&self) -> u32 {
        self.bytes.len()
    }

    /// Returns the immutable logical capacity selected at construction.
    pub fn capacity(&self) -> u32 {
        self.bytes.capacity()
    }

    /// Reports whether the owner contains no bytes.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Transfers the owned bytes to the caller.
    pub fn into_vec(self) -> Vec<u8> {
        self.bytes.into_vec()
    }
}

/// A byte sequence with a maximum stored length of 64 bytes.
#[derive(Debug)]
pub struct BoundedBytes64([u8; 64], u8);

impl BoundedBytes64 {
    /// Copies a byte sequence of at most 64 bytes into this fixed owner.
    pub fn try_from_slice(bytes: &[u8]) -> Result<Self, BoundedOwnerError> {
        if bytes.len() > 64 {
            return Err(BoundedOwnerError::LengthExceedsCapacity);
        }
        let mut storage = [0; 64];
        storage[..bytes.len()].copy_from_slice(bytes);
        Ok(Self(storage, bytes.len() as u8))
    }

    /// Returns the stored bytes.
    pub fn as_slice(&self) -> &[u8] {
        &self.0[..usize::from(self.1)]
    }

    /// Returns the exact stored byte length.
    pub fn len(&self) -> u8 {
        self.1
    }

    /// Reports whether the owner contains no bytes.
    pub fn is_empty(&self) -> bool {
        self.1 == 0
    }
}

/// A validated UTF-8 owner whose byte length may not exceed `N`.
#[derive(Debug)]
pub struct BoundedUtf8<const N: usize> {
    value: String,
}

impl<const N: usize> BoundedUtf8<N> {
    /// Takes valid UTF-8 bytes whose length is at most `N`.
    pub fn try_from_bytes(bytes: Vec<u8>) -> Result<Self, BytesConstructionRejected> {
        if bytes.len() > N {
            return Err(BytesConstructionRejected {
                bytes,
                reason: BoundedOwnerError::LengthExceedsCapacity,
            });
        }
        String::from_utf8(bytes)
            .map(|value| Self { value })
            .map_err(|error| BytesConstructionRejected {
                bytes: error.into_bytes(),
                reason: BoundedOwnerError::InvalidUtf8,
            })
    }

    /// Returns the validated UTF-8 text.
    pub fn as_str(&self) -> &str {
        &self.value
    }

    /// Returns the validated UTF-8 bytes.
    pub fn as_bytes(&self) -> &[u8] {
        self.value.as_bytes()
    }

    /// Returns the exact UTF-8 byte length.
    pub fn len(&self) -> usize {
        self.value.len()
    }

    /// Reports whether the owner contains no bytes.
    pub fn is_empty(&self) -> bool {
        self.value.is_empty()
    }

    /// Transfers the owned validated bytes to the caller.
    pub fn into_bytes(self) -> Vec<u8> {
        self.value.into_bytes()
    }
}

/// Immutable, shareable bytes whose length is checked before ownership transfers.
#[derive(Debug)]
pub struct OwnedBytes {
    bytes: Arc<[u8]>,
    length: u64,
}

impl OwnedBytes {
    /// Takes `bytes` only when their exact length does not exceed `max_bytes`.
    pub fn try_from_vec(bytes: Vec<u8>, max_bytes: u64) -> Result<Self, BytesConstructionRejected> {
        let length = u64::try_from(bytes.len()).expect("usize is representable as u64");
        if length > max_bytes {
            return Err(BytesConstructionRejected {
                bytes,
                reason: BoundedOwnerError::LengthExceedsCapacity,
            });
        }
        Ok(Self {
            bytes: Arc::from(bytes),
            length,
        })
    }

    /// Returns the immutable bytes.
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the exact immutable byte length.
    pub fn len(&self) -> u64 {
        self.length
    }

    /// Reports whether the owner contains no bytes.
    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    /// Transfers the immutable allocation or share to the caller.
    pub fn into_arc(self) -> Arc<[u8]> {
        self.bytes
    }
}

fn try_allocate<T>(capacity: u32) -> Result<Vec<T>, BoundedOwnerError> {
    try_allocate_with(capacity, |values, capacity| {
        values.try_reserve_exact(capacity).map_err(|_| ())
    })
}

fn try_allocate_with<T>(
    capacity: u32,
    reserve: impl FnOnce(&mut Vec<T>, usize) -> Result<(), ()>,
) -> Result<Vec<T>, BoundedOwnerError> {
    let capacity = usize::try_from(capacity).map_err(|_| BoundedOwnerError::CapacityTooLarge)?;
    let element_size = size_of::<T>();
    if element_size != 0 && capacity > (isize::MAX as usize) / element_size {
        return Err(BoundedOwnerError::CapacityTooLarge);
    }
    let mut values = Vec::new();
    reserve(&mut values, capacity).map_err(|_| BoundedOwnerError::AllocationFailed)?;
    Ok(values)
}

fn fits_capacity(length: usize, capacity: u32) -> bool {
    u32::try_from(length).is_ok_and(|length| length <= capacity)
}
