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
    fn zero_capacity_owners_accept_only_empty_values() {
        let values = BoundedVec::<u8>::try_with_capacity(0).unwrap();
        assert!(values.is_empty());
        assert_eq!(values.capacity(), 0);
        assert_eq!(values.into_vec(), Vec::<u8>::new());

        let values = BoundedVec::<u8>::try_from_vec(Vec::new(), 0).unwrap();
        assert!(values.is_empty());
        assert_eq!(values.capacity(), 0);

        let bytes = BoundedBytes::try_from_vec(Vec::new(), 0).unwrap();
        assert!(bytes.is_empty());
        assert_eq!(bytes.capacity(), 0);
        assert_eq!(bytes.into_vec(), Vec::<u8>::new());

        let rejected = BoundedBytes::try_from_vec(vec![1], 0).unwrap_err();
        assert_eq!(rejected.reason, BoundedOwnerError::LengthExceedsCapacity);
        assert_eq!(rejected.bytes, vec![1]);
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
        let empty = BoundedBytes64::try_from_slice(&[]).unwrap();
        assert!(empty.is_empty());

        let accepted_64 = [7; 64];
        let fixed = BoundedBytes64::try_from_slice(&accepted_64).unwrap();
        assert_eq!(fixed.len(), 64);
        assert_eq!(fixed.as_slice(), accepted_64);

        assert!(matches!(
            BoundedBytes64::try_from_slice(&[0; 65]),
            Err(BoundedOwnerError::LengthExceedsCapacity)
        ));

        let invalid = BoundedUtf8::<8>::try_from_bytes(vec![0xff]).unwrap_err();
        assert_eq!(invalid.reason, BoundedOwnerError::InvalidUtf8);
        assert_eq!(invalid.bytes, vec![0xff]);

        let oversized = BoundedUtf8::<2>::try_from_bytes(b"abc".to_vec()).unwrap_err();
        assert_eq!(oversized.reason, BoundedOwnerError::LengthExceedsCapacity);
        assert_eq!(oversized.bytes, b"abc");

        let text = BoundedUtf8::<4>::try_from_bytes("mori".into()).unwrap();
        assert_eq!(text.len(), 4);
        assert_eq!(text.as_str(), "mori");
        assert_eq!(text.into_bytes(), b"mori");
    }

    #[test]
    fn owners_iterate_and_consume_in_order() {
        let values = BoundedVec::try_from_vec(vec![3, 1, 4], 3).unwrap();
        assert_eq!(values.iter().copied().collect::<Vec<_>>(), vec![3, 1, 4]);
        assert_eq!(values.into_vec(), vec![3, 1, 4]);

        let bytes = BoundedBytes::try_from_vec(vec![9, 10], 2).unwrap();
        assert_eq!(bytes.as_slice(), [9, 10]);
        assert_eq!(bytes.into_vec(), vec![9, 10]);
    }

    #[test]
    fn immutable_bytes_preserve_exact_content_and_share_allocation() {
        let bytes = OwnedBytes::try_from_vec(vec![9, 10], 2).unwrap();
        assert_eq!(bytes.len(), 2);
        assert_eq!(bytes.as_slice(), [9, 10]);
        let shared = bytes.clone();
        let bytes = bytes.into_arc();
        let shared = shared.into_arc();
        assert!(triomphe::Arc::ptr_eq(&bytes, &shared));
        assert_eq!(&*bytes, [9, 10]);

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

    #[test]
    fn immutable_byte_allocation_failure_returns_original_bytes() {
        let rejected = super::try_shared_owned_bytes_with(vec![9, 10], 2, |_| Err(())).unwrap_err();
        assert_eq!(rejected.reason, BoundedOwnerError::AllocationFailed);
        assert_eq!(rejected.bytes, vec![9, 10]);
    }

    #[test]
    fn capacity_comparisons_handle_zero_and_usize_boundaries() {
        assert!(super::fits_capacity(0, 0));
        assert!(super::fits_capacity(
            usize::try_from(u32::MAX).unwrap(),
            u32::MAX
        ));
        assert!(!super::fits_capacity(usize::MAX, u32::MAX));
    }
}
use super::SharedArc;
use std::mem::size_of;

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
///
/// Its logical capacity is a `u32`, even if its backing allocation has room for
/// more elements. Methods never expose mutable access to that allocation.
#[derive(Debug)]
pub struct BoundedVec<T> {
    values: Vec<T>,
    capacity: u32,
}

impl<T> BoundedVec<T> {
    /// Allocates an empty owner with exactly `capacity` permitted values.
    ///
    /// Returns [`BoundedOwnerError::CapacityTooLarge`] when the request cannot
    /// be represented as a finite allocation, or
    /// [`BoundedOwnerError::AllocationFailed`] when it cannot be reserved.
    ///
    /// ```
    /// use moria::prelude::BoundedVec;
    ///
    /// let values = BoundedVec::<u8>::try_with_capacity(2)?;
    /// assert_eq!(values.capacity(), 2);
    /// # Ok::<(), moria::prelude::BoundedOwnerError>(())
    /// ```
    pub fn try_with_capacity(capacity: u32) -> Result<Self, BoundedOwnerError> {
        Ok(Self {
            values: try_allocate(capacity)?,
            capacity,
        })
    }

    /// Takes `values` only when their count fits the declared capacity.
    ///
    /// Returns the original vector with the rejection reason if its length
    /// exceeds `capacity` or allocating the finite owner fails.
    ///
    /// ```
    /// use moria::prelude::BoundedVec;
    ///
    /// assert_eq!(BoundedVec::try_from_vec(vec![1, 2], 2)?.as_slice(), [1, 2]);
    /// # Ok::<(), moria::prelude::VecConstructionRejected<u8>>(())
    /// ```
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
    ///
    /// Returns the uninserted value with
    /// [`BoundedOwnerError::LengthExceedsCapacity`] without mutating this owner
    /// when it is already at its fixed capacity.
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
    ///
    /// Returns [`BoundedOwnerError::CapacityTooLarge`] or
    /// [`BoundedOwnerError::AllocationFailed`] when the requested finite
    /// allocation cannot be created.
    ///
    /// ```
    /// use moria::prelude::BoundedBytes;
    ///
    /// assert!(BoundedBytes::try_with_capacity(0)?.is_empty());
    /// # Ok::<(), moria::prelude::BoundedOwnerError>(())
    /// ```
    pub fn try_with_capacity(capacity: u32) -> Result<Self, BoundedOwnerError> {
        BoundedVec::try_with_capacity(capacity).map(|bytes| Self { bytes })
    }

    /// Takes `bytes` only when their length fits the declared capacity.
    ///
    /// Returns the original bytes on a length, capacity, or allocation failure.
    ///
    /// ```
    /// use moria::prelude::BoundedBytes;
    ///
    /// assert_eq!(BoundedBytes::try_from_vec(vec![1, 2], 2)?.as_slice(), [1, 2]);
    /// # Ok::<(), moria::prelude::BytesConstructionRejected>(())
    /// ```
    pub fn try_from_vec(bytes: Vec<u8>, capacity: u32) -> Result<Self, BytesConstructionRejected> {
        BoundedVec::try_from_vec(bytes, capacity)
            .map(|bytes| Self { bytes })
            .map_err(|rejected| BytesConstructionRejected {
                bytes: rejected.values,
                reason: rejected.reason,
            })
    }

    /// Appends all bytes or leaves this owner unchanged when they will not fit.
    ///
    /// Returns [`BoundedOwnerError::LengthExceedsCapacity`] and performs no
    /// partial append if the checked resulting length exceeds this owner's
    /// fixed capacity.
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
    ///
    /// Returns [`BoundedOwnerError::LengthExceedsCapacity`] for inputs longer
    /// than 64 bytes.
    ///
    /// ```
    /// use moria::prelude::BoundedBytes64;
    ///
    /// assert_eq!(BoundedBytes64::try_from_slice(&[7; 64])?.len(), 64);
    /// # Ok::<(), moria::prelude::BoundedOwnerError>(())
    /// ```
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
    ///
    /// Returns the original bytes with [`BoundedOwnerError::LengthExceedsCapacity`]
    /// when the byte length is greater than `N`, or with
    /// [`BoundedOwnerError::InvalidUtf8`] when they are not valid UTF-8.
    ///
    /// ```
    /// use moria::prelude::BoundedUtf8;
    ///
    /// assert_eq!(BoundedUtf8::<4>::try_from_bytes("mori".into())?.as_str(), "mori");
    /// # Ok::<(), moria::prelude::BytesConstructionRejected>(())
    /// ```
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
///
/// The stored allocation is shared through a [`SharedArc`] slice. Clones share the
/// same immutable allocation, and no spare growable capacity is exposed.
#[derive(Clone, Debug)]
pub struct OwnedBytes {
    bytes: SharedArc<[u8]>,
    length: u64,
}

impl OwnedBytes {
    /// Takes `bytes` only when their exact length does not exceed `max_bytes`.
    ///
    /// Returns the original bytes with [`BoundedOwnerError::LengthExceedsCapacity`]
    /// when `max_bytes` is smaller than their length, or with
    /// [`BoundedOwnerError::AllocationFailed`] if creating the shared slice
    /// allocation fails.
    ///
    /// ```
    /// use moria::prelude::OwnedBytes;
    ///
    /// let bytes = OwnedBytes::try_from_vec(vec![1, 2], 2)?;
    /// assert_eq!(&*bytes.clone().into_arc(), [1, 2]);
    /// # Ok::<(), moria::prelude::BytesConstructionRejected>(())
    /// ```
    pub fn try_from_vec(bytes: Vec<u8>, max_bytes: u64) -> Result<Self, BytesConstructionRejected> {
        let length = u64::try_from(bytes.len()).expect("usize is representable as u64");
        if length > max_bytes {
            return Err(BytesConstructionRejected {
                bytes,
                reason: BoundedOwnerError::LengthExceedsCapacity,
            });
        }
        try_shared_owned_bytes_with(bytes, length, |bytes| {
            SharedArc::try_from_header_and_slice((), bytes)
                .map(Into::into)
                .map_err(|_| ())
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

    /// Transfers the immutable shared root to the caller.
    pub fn into_arc(self) -> SharedArc<[u8]> {
        self.bytes
    }
}

fn try_shared_owned_bytes_with(
    bytes: Vec<u8>,
    length: u64,
    allocate: impl FnOnce(&[u8]) -> Result<SharedArc<[u8]>, ()>,
) -> Result<OwnedBytes, BytesConstructionRejected> {
    let root = match allocate(&bytes) {
        Ok(root) => root,
        Err(()) => {
            return Err(BytesConstructionRejected {
                bytes,
                reason: BoundedOwnerError::AllocationFailed,
            });
        }
    };
    Ok(OwnedBytes {
        bytes: root,
        length,
    })
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
