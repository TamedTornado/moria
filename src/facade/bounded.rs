//! Finite, lossless owners shared by the public facade.

use std::sync::Arc;

/// The reason a finite owner could not accept supplied data.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundedOwnerError {
    /// The requested capacity cannot be represented by the host allocation API.
    CapacityTooLarge,
    /// The supplied value has more elements or bytes than its admitted capacity.
    LengthExceedsCapacity,
    /// The supplied byte sequence is not valid UTF-8.
    InvalidUtf8,
    /// Reserving the required finite allocation failed.
    AllocationFailed,
}

/// A vector returned unchanged when a bounded-vector construction is rejected.
#[derive(Debug)]
pub struct VecConstructionRejected<T> {
    /// The exact caller allocation supplied to construction.
    pub values: Vec<T>,
    /// The rejection reason.
    pub reason: BoundedOwnerError,
}

/// Bytes returned unchanged when byte-owner construction is rejected.
#[derive(Debug)]
pub struct BytesConstructionRejected {
    /// The exact caller allocation supplied to construction.
    pub bytes: Vec<u8>,
    /// The rejection reason.
    pub reason: BoundedOwnerError,
}

/// An element returned unchanged when appending to a full bounded vector fails.
#[derive(Debug)]
pub struct BoundedPushRejected<T> {
    /// The rejected element.
    pub value: T,
    /// The rejection reason.
    pub reason: BoundedOwnerError,
}

/// An owned vector with an immutable admitted element capacity.
///
/// Construction reserves its complete capacity before accepting values. Calls
/// that fail to construct or append return the supplied allocation or element
/// unchanged; the owner never reallocates after construction.
///
/// # Examples
///
/// ```
/// use moria::facade::BoundedVec;
///
/// let values = BoundedVec::try_from_vec(vec![1, 2], 2)?;
/// assert_eq!(values.as_slice(), &[1, 2]);
/// # Ok::<(), moria::facade::VecConstructionRejected<i32>>(())
/// ```
#[derive(Debug)]
pub struct BoundedVec<T> {
    values: Vec<T>,
    capacity: u32,
}

impl<T> BoundedVec<T> {
    /// Creates an empty vector with exactly `capacity` admitted element slots.
    ///
    /// # Errors
    ///
    /// Returns [`BoundedOwnerError::AllocationFailed`] when the finite backing
    /// allocation cannot be reserved.
    pub fn try_with_capacity(capacity: u32) -> Result<Self, BoundedOwnerError> {
        let values = reserve_capacity(capacity)?;
        Ok(Self { values, capacity })
    }

    /// Accepts `values` only when they fit within `capacity`.
    ///
    /// # Errors
    ///
    /// Returns the original `values` unchanged on rejection, including an
    /// allocation failure.
    pub fn try_from_vec(
        mut values: Vec<T>,
        capacity: u32,
    ) -> Result<Self, VecConstructionRejected<T>> {
        if values.len() > usize::try_from(capacity).expect("u32 fits usize") {
            return Err(VecConstructionRejected {
                values,
                reason: BoundedOwnerError::LengthExceedsCapacity,
            });
        }

        let mut owned = match reserve_capacity(capacity) {
            Ok(owned) => owned,
            Err(reason) => return Err(VecConstructionRejected { values, reason }),
        };
        owned.append(&mut values);
        Ok(Self {
            values: owned,
            capacity,
        })
    }

    /// Appends one value when an admitted element slot remains.
    ///
    /// # Errors
    ///
    /// Returns the supplied value unchanged when this vector is already full.
    pub fn try_push(&mut self, value: T) -> Result<(), BoundedPushRejected<T>> {
        if self.values.len() == usize::try_from(self.capacity).expect("u32 fits usize") {
            return Err(BoundedPushRejected {
                value,
                reason: BoundedOwnerError::LengthExceedsCapacity,
            });
        }
        self.values.push(value);
        Ok(())
    }

    /// Borrows the accepted elements in insertion order.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        &self.values
    }

    /// Iterates over the accepted elements in insertion order.
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.values.iter()
    }

    /// Returns the exact accepted element count.
    #[must_use]
    pub fn len(&self) -> u32 {
        u32::try_from(self.values.len()).expect("bounded vector length fits u32")
    }

    /// Returns the immutable admitted element capacity.
    #[must_use]
    pub const fn capacity(&self) -> u32 {
        self.capacity
    }

    /// Reports whether no values have been accepted.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Transfers the accepted values in insertion order.
    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        self.values
    }
}

/// An owned byte vector with an immutable admitted byte capacity.
///
/// # Examples
///
/// ```
/// use moria::facade::BoundedBytes;
///
/// let bytes = BoundedBytes::try_from_vec(vec![1, 2], 2)?;
/// assert_eq!(bytes.as_slice(), &[1, 2]);
/// # Ok::<(), moria::facade::BytesConstructionRejected>(())
/// ```
#[derive(Debug)]
pub struct BoundedBytes {
    bytes: Vec<u8>,
    capacity: u32,
}

impl BoundedBytes {
    /// Creates empty bytes with exactly `capacity` admitted byte slots.
    pub fn try_with_capacity(capacity: u32) -> Result<Self, BoundedOwnerError> {
        let bytes = reserve_capacity(capacity)?;
        Ok(Self { bytes, capacity })
    }

    /// Accepts `bytes` only when they fit within `capacity`.
    ///
    /// # Errors
    ///
    /// Returns the original bytes unchanged on rejection.
    pub fn try_from_vec(
        mut bytes: Vec<u8>,
        capacity: u32,
    ) -> Result<Self, BytesConstructionRejected> {
        if bytes.len() > usize::try_from(capacity).expect("u32 fits usize") {
            return Err(BytesConstructionRejected {
                bytes,
                reason: BoundedOwnerError::LengthExceedsCapacity,
            });
        }

        let mut owned = match reserve_capacity(capacity) {
            Ok(owned) => owned,
            Err(reason) => return Err(BytesConstructionRejected { bytes, reason }),
        };
        owned.append(&mut bytes);
        Ok(Self {
            bytes: owned,
            capacity,
        })
    }

    /// Appends all bytes or appends none when the complete slice does not fit.
    ///
    /// # Errors
    ///
    /// Returns [`BoundedOwnerError::LengthExceedsCapacity`] without modifying
    /// this owner if the whole input slice cannot fit.
    pub fn try_extend_from_slice(&mut self, bytes: &[u8]) -> Result<(), BoundedOwnerError> {
        let remaining = usize::try_from(self.capacity).expect("u32 fits usize") - self.bytes.len();
        if bytes.len() > remaining {
            return Err(BoundedOwnerError::LengthExceedsCapacity);
        }
        self.bytes.extend_from_slice(bytes);
        Ok(())
    }

    /// Borrows the accepted bytes.
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the exact accepted byte length.
    #[must_use]
    pub fn len(&self) -> u32 {
        u32::try_from(self.bytes.len()).expect("bounded byte length fits u32")
    }

    /// Returns the immutable admitted byte capacity.
    #[must_use]
    pub const fn capacity(&self) -> u32 {
        self.capacity
    }

    /// Reports whether no bytes have been accepted.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Transfers the accepted bytes.
    #[must_use]
    pub fn into_vec(self) -> Vec<u8> {
        self.bytes
    }
}

/// Up to 64 bytes stored inline with an exact stored length.
///
/// # Examples
///
/// ```
/// use moria::facade::BoundedBytes64;
///
/// let bytes = BoundedBytes64::try_from_slice(b"moria")?;
/// assert_eq!(bytes.len(), 5);
/// # Ok::<(), moria::facade::BoundedOwnerError>(())
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundedBytes64([u8; 64], u8);

impl BoundedBytes64 {
    /// Copies at most 64 bytes into fixed inline storage.
    ///
    /// # Errors
    ///
    /// Returns [`BoundedOwnerError::LengthExceedsCapacity`] when `bytes` has
    /// more than 64 bytes.
    pub fn try_from_slice(bytes: &[u8]) -> Result<Self, BoundedOwnerError> {
        let length =
            u8::try_from(bytes.len()).map_err(|_| BoundedOwnerError::LengthExceedsCapacity)?;
        if bytes.len() > 64 {
            return Err(BoundedOwnerError::LengthExceedsCapacity);
        }
        let mut stored = [0; 64];
        stored[..bytes.len()].copy_from_slice(bytes);
        Ok(Self(stored, length))
    }

    /// Borrows the exact stored byte prefix.
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.0[..usize::from(self.1)]
    }

    /// Returns the exact stored byte length.
    #[must_use]
    pub const fn len(&self) -> u8 {
        self.1
    }

    /// Reports whether no bytes are stored.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.1 == 0
    }
}

/// Valid UTF-8 whose byte length cannot exceed `N`.
///
/// # Examples
///
/// ```
/// use moria::facade::BoundedUtf8;
///
/// let text = BoundedUtf8::<5>::try_from_bytes(b"moria".to_vec())?;
/// assert_eq!(text.as_str(), "moria");
/// # Ok::<(), moria::facade::BytesConstructionRejected>(())
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundedUtf8<const N: usize> {
    text: String,
}

impl<const N: usize> BoundedUtf8<N> {
    /// Accepts valid UTF-8 of at most `N` bytes.
    ///
    /// # Errors
    ///
    /// Returns the original bytes unchanged if they are invalid UTF-8 or are
    /// longer than `N`, and returns them unchanged with
    /// [`BoundedOwnerError::AllocationFailed`] if normalized backing storage
    /// cannot be reserved. Accepted text owns an exact-length backing buffer,
    /// so [`Self::into_bytes`] cannot expose the caller's spare capacity.
    pub fn try_from_bytes(bytes: Vec<u8>) -> Result<Self, BytesConstructionRejected> {
        if bytes.len() > N {
            return Err(BytesConstructionRejected {
                bytes,
                reason: BoundedOwnerError::LengthExceedsCapacity,
            });
        }
        if std::str::from_utf8(&bytes).is_err() {
            return Err(BytesConstructionRejected {
                bytes,
                reason: BoundedOwnerError::InvalidUtf8,
            });
        }

        let normalized = match copy_into_exact_backing(&bytes) {
            Ok(normalized) => normalized,
            Err(reason) => return Err(BytesConstructionRejected { bytes, reason }),
        };
        let text = String::from_utf8(normalized).expect("validated UTF-8 is accepted by String");
        Ok(Self { text })
    }

    /// Borrows the validated UTF-8 text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.text
    }

    /// Borrows the validated UTF-8 bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        self.text.as_bytes()
    }

    /// Returns the exact UTF-8 byte length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.text.len()
    }

    /// Reports whether the text is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    /// Transfers the validated UTF-8 bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.text.into_bytes()
    }
}

/// Immutable bytes with an exact admitted `u64` length.
///
/// Clones share the same private immutable backing, so cloning does not copy
/// the bytes or allocate another full-size byte buffer. Construction first
/// copies into exact-length backing with the fallible standard allocation path,
/// preserving the caller allocation on failure.
///
/// # Examples
///
/// ```
/// use moria::facade::OwnedBytes;
///
/// let bytes = OwnedBytes::try_from_vec(vec![1, 2, 3], 3)?;
/// let shared = bytes.clone();
/// assert_eq!(shared.as_slice(), &[1, 2, 3]);
/// # Ok::<(), moria::facade::BytesConstructionRejected>(())
/// ```
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct OwnedBytes {
    bytes: Arc<Vec<u8>>,
}

impl OwnedBytes {
    /// Accepts bytes whose exact length is at most `max_bytes`.
    ///
    /// # Errors
    ///
    /// Returns the original bytes unchanged if they exceed `max_bytes` or the
    /// immutable backing allocation cannot be reserved. On success, clones
    /// share the same exact-length immutable backing allocation.
    pub fn try_from_vec(bytes: Vec<u8>, max_bytes: u64) -> Result<Self, BytesConstructionRejected> {
        if u64::try_from(bytes.len()).expect("usize fits u64") > max_bytes {
            return Err(BytesConstructionRejected {
                bytes,
                reason: BoundedOwnerError::LengthExceedsCapacity,
            });
        }

        let immutable = match copy_into_exact_backing(&bytes) {
            Ok(immutable) => immutable,
            Err(reason) => return Err(BytesConstructionRejected { bytes, reason }),
        };
        Ok(Self {
            bytes: Arc::new(immutable),
        })
    }

    /// Borrows the immutable bytes.
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    /// Returns the exact immutable byte length.
    #[must_use]
    pub fn len(&self) -> u64 {
        u64::try_from(self.bytes.len()).expect("usize fits u64")
    }

    /// Reports whether no bytes are stored.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
}

fn reserve_capacity<T>(capacity: u32) -> Result<Vec<T>, BoundedOwnerError> {
    let capacity = usize::try_from(capacity).map_err(|_| BoundedOwnerError::CapacityTooLarge)?;
    let element_size = std::mem::size_of::<T>();
    if element_size != 0 && capacity > (isize::MAX as usize) / element_size {
        return Err(BoundedOwnerError::CapacityTooLarge);
    }

    let mut values = Vec::new();
    values
        .try_reserve_exact(capacity)
        .map_err(|_| BoundedOwnerError::AllocationFailed)?;
    Ok(values)
}

fn copy_into_exact_backing(bytes: &[u8]) -> Result<Vec<u8>, BoundedOwnerError> {
    if allocation_failure_requested() {
        return Err(BoundedOwnerError::AllocationFailed);
    }

    let mut backing = Vec::new();
    backing
        .try_reserve_exact(bytes.len())
        .map_err(|_| BoundedOwnerError::AllocationFailed)?;
    backing.extend_from_slice(bytes);
    Ok(backing)
}

#[cfg(test)]
thread_local! {
    static FORCE_ALLOCATION_FAILURE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

#[cfg(test)]
fn allocation_failure_requested() -> bool {
    FORCE_ALLOCATION_FAILURE.replace(false)
}

#[cfg(not(test))]
fn allocation_failure_requested() -> bool {
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero_capacity_owners_are_empty_and_reject_growth() {
        let mut values = BoundedVec::<u8>::try_with_capacity(0).unwrap();
        assert_eq!(values.try_push(7).unwrap_err().value, 7);

        let mut bytes = BoundedBytes::try_with_capacity(0).unwrap();
        assert_eq!(
            bytes.try_extend_from_slice(&[7]),
            Err(BoundedOwnerError::LengthExceedsCapacity)
        );
        assert!(bytes.is_empty());
    }

    #[test]
    fn unrepresentable_allocation_size_is_reported_without_attempting_allocation() {
        type Huge = [u8; 4_294_967_296];

        assert!(matches!(
            BoundedVec::<Huge>::try_with_capacity(u32::MAX),
            Err(BoundedOwnerError::CapacityTooLarge)
        ));
    }

    #[test]
    fn allocation_failure_recovers_the_original_byte_allocation() {
        let mut bytes = Vec::with_capacity(128);
        bytes.extend_from_slice(b"moria");
        let input_pointer = bytes.as_ptr();
        let input_capacity = bytes.capacity();

        FORCE_ALLOCATION_FAILURE.set(true);
        let rejected = OwnedBytes::try_from_vec(bytes, 5).unwrap_err();

        assert_eq!(rejected.reason, BoundedOwnerError::AllocationFailed);
        assert_eq!(rejected.bytes, b"moria");
        assert_eq!(rejected.bytes.as_ptr(), input_pointer);
        assert_eq!(rejected.bytes.capacity(), input_capacity);
    }
}
