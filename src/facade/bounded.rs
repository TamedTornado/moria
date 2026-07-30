//! Lossless finite owners used by the public facade.

use arc_slice::ArcBytes;

/// A reason a finite owner could not be constructed or extended.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BoundedOwnerError {
    /// The declared capacity cannot be represented by an allocation for this element type.
    CapacityTooLarge,
    /// The supplied length would exceed the declared immutable capacity.
    LengthExceedsCapacity,
    /// The supplied bytes are not well-formed UTF-8.
    InvalidUtf8,
    /// A checked allocation could not be reserved.
    AllocationFailed,
}

/// A vector constructor failure that returns the caller's original values.
#[derive(Debug, Eq, PartialEq)]
pub struct VecConstructionRejected<T> {
    /// The values supplied to the constructor, unchanged.
    pub values: Vec<T>,
    /// Why construction was rejected.
    pub reason: BoundedOwnerError,
}

/// A byte-owner constructor failure that returns the caller's original bytes.
#[derive(Debug, Eq, PartialEq)]
pub struct BytesConstructionRejected {
    /// The bytes supplied to the constructor, unchanged.
    pub bytes: Vec<u8>,
    /// Why construction was rejected.
    pub reason: BoundedOwnerError,
}

/// A rejected bounded-vector append that returns the uninserted value.
#[derive(Debug, Eq, PartialEq)]
pub struct BoundedPushRejected<T> {
    /// The value that was not appended.
    pub value: T,
    /// Why the append was rejected.
    pub reason: BoundedOwnerError,
}

/// An owned vector with an immutable, exact public element capacity.
///
/// Construction reserves the requested finite capacity before ownership is
/// accepted. Appending beyond that capacity returns the original value.
#[derive(Debug)]
pub struct BoundedVec<T> {
    values: Vec<T>,
    capacity: u32,
}

impl<T> BoundedVec<T> {
    /// Creates an empty vector with `capacity` permitted elements.
    ///
    /// # Errors
    ///
    /// Returns [`BoundedOwnerError::CapacityTooLarge`] when `capacity` cannot
    /// represent an allocation of `T`, or [`BoundedOwnerError::AllocationFailed`]
    /// when the allocation cannot be reserved.
    ///
    /// # Examples
    ///
    /// ```
    /// use moria::facade::BoundedVec;
    ///
    /// let values = BoundedVec::<u8>::try_with_capacity(2).unwrap();
    /// assert_eq!(values.capacity(), 2);
    /// ```
    pub fn try_with_capacity(capacity: u32) -> Result<Self, BoundedOwnerError> {
        let requested = usize::try_from(capacity).expect("u32 always fits usize");
        ensure_element_capacity::<T>(requested)?;
        let mut values = Vec::new();
        values
            .try_reserve_exact(requested)
            .map_err(|_| BoundedOwnerError::AllocationFailed)?;
        Ok(Self { values, capacity })
    }

    /// Accepts `values` only when their exact count fits `capacity`.
    ///
    /// # Errors
    ///
    /// On rejection the returned [`VecConstructionRejected`] contains the
    /// original vector allocation and the exact reason; no values are dropped.
    pub fn try_from_vec(values: Vec<T>, capacity: u32) -> Result<Self, VecConstructionRejected<T>> {
        let requested = usize::try_from(capacity).expect("u32 always fits usize");
        if values.len() > requested {
            return Err(VecConstructionRejected {
                values,
                reason: BoundedOwnerError::LengthExceedsCapacity,
            });
        }
        if let Err(reason) = ensure_element_capacity::<T>(requested) {
            return Err(VecConstructionRejected { values, reason });
        }
        let mut accepted = Vec::new();
        if accepted.try_reserve_exact(requested).is_err() {
            return Err(VecConstructionRejected {
                values,
                reason: BoundedOwnerError::AllocationFailed,
            });
        }
        accepted.extend(values);
        Ok(Self {
            values: accepted,
            capacity,
        })
    }

    /// Appends one value if the exact fixed capacity has room.
    ///
    /// # Errors
    ///
    /// A full owner returns [`BoundedPushRejected`] containing `value`, leaving
    /// this owner unchanged.
    pub fn try_push(&mut self, value: T) -> Result<(), BoundedPushRejected<T>> {
        if self.values.len() == usize::try_from(self.capacity).expect("u32 always fits usize") {
            return Err(BoundedPushRejected {
                value,
                reason: BoundedOwnerError::LengthExceedsCapacity,
            });
        }
        self.values.push(value);
        Ok(())
    }

    /// Borrows the accepted values in insertion order.
    #[must_use]
    pub fn as_slice(&self) -> &[T] {
        &self.values
    }

    /// Iterates over the accepted values in insertion order.
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.values.iter()
    }

    /// Returns the exact accepted element count.
    #[must_use]
    pub fn len(&self) -> u32 {
        u32::try_from(self.values.len()).expect("bounded vector length fits u32")
    }

    /// Returns the immutable public element capacity.
    #[must_use]
    pub const fn capacity(&self) -> u32 {
        self.capacity
    }

    /// Reports whether the vector has no accepted values.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Consumes the owner and returns its accepted allocation.
    #[must_use]
    pub fn into_vec(self) -> Vec<T> {
        self.values
    }
}

/// An owned byte vector with an immutable, exact public byte capacity.
#[derive(Debug)]
pub struct BoundedBytes {
    bytes: BoundedVec<u8>,
}

impl BoundedBytes {
    /// Creates an empty byte owner with `capacity` permitted bytes.
    ///
    /// # Errors
    ///
    /// Returns the checked allocation error described by
    /// [`BoundedVec::try_with_capacity`].
    pub fn try_with_capacity(capacity: u32) -> Result<Self, BoundedOwnerError> {
        BoundedVec::try_with_capacity(capacity).map(|bytes| Self { bytes })
    }

    /// Accepts `bytes` only when their exact length fits `capacity`.
    ///
    /// # Errors
    ///
    /// A rejection returns the original byte allocation unchanged.
    pub fn try_from_vec(bytes: Vec<u8>, capacity: u32) -> Result<Self, BytesConstructionRejected> {
        BoundedVec::try_from_vec(bytes, capacity)
            .map(|bytes| Self { bytes })
            .map_err(|rejected| BytesConstructionRejected {
                bytes: rejected.values,
                reason: rejected.reason,
            })
    }

    /// Appends all `bytes`, or appends none when they do not fit.
    ///
    /// # Errors
    ///
    /// Returns [`BoundedOwnerError::LengthExceedsCapacity`] without changing
    /// this owner if the entire slice cannot fit.
    pub fn try_extend_from_slice(&mut self, bytes: &[u8]) -> Result<(), BoundedOwnerError> {
        let current = self.bytes.values.len();
        let capacity = usize::try_from(self.bytes.capacity).expect("u32 always fits usize");
        let Some(new_len) = current.checked_add(bytes.len()) else {
            return Err(BoundedOwnerError::LengthExceedsCapacity);
        };
        if new_len > capacity {
            return Err(BoundedOwnerError::LengthExceedsCapacity);
        }
        self.bytes.values.extend_from_slice(bytes);
        Ok(())
    }

    /// Borrows the accepted bytes.
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        self.bytes.as_slice()
    }

    /// Returns the exact accepted byte count.
    #[must_use]
    pub fn len(&self) -> u32 {
        self.bytes.len()
    }

    /// Returns the immutable public byte capacity.
    #[must_use]
    pub const fn capacity(&self) -> u32 {
        self.bytes.capacity()
    }

    /// Reports whether the owner contains no bytes.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Consumes the owner and returns its accepted allocation.
    #[must_use]
    pub fn into_vec(self) -> Vec<u8> {
        self.bytes.into_vec()
    }
}

/// Up to 64 uninterpreted bytes stored inline.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundedBytes64([u8; 64], u8);

impl BoundedBytes64 {
    /// Copies at most 64 bytes into an inline owner.
    ///
    /// # Errors
    ///
    /// Returns [`BoundedOwnerError::LengthExceedsCapacity`] when `bytes` is
    /// longer than 64 bytes.
    pub fn try_from_slice(bytes: &[u8]) -> Result<Self, BoundedOwnerError> {
        let length =
            u8::try_from(bytes.len()).map_err(|_| BoundedOwnerError::LengthExceedsCapacity)?;
        if length > 64 {
            return Err(BoundedOwnerError::LengthExceedsCapacity);
        }
        let mut stored = [0; 64];
        stored[..usize::from(length)].copy_from_slice(bytes);
        Ok(Self(stored, length))
    }

    /// Borrows the exact accepted prefix.
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.0[..usize::from(self.1)]
    }

    /// Returns the exact accepted byte count.
    #[must_use]
    pub const fn len(&self) -> u8 {
        self.1
    }

    /// Reports whether no bytes were accepted.
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.1 == 0
    }
}

/// Valid UTF-8 bytes with an immutable maximum byte length `N`.
#[derive(Debug)]
pub struct BoundedUtf8<const N: usize> {
    bytes: Vec<u8>,
}

impl<const N: usize> BoundedUtf8<N> {
    /// Validates and accepts UTF-8 whose byte length is at most `N`.
    ///
    /// # Errors
    ///
    /// Rejection returns the original bytes unchanged for an oversized or
    /// invalid UTF-8 input, or when the bounded replacement allocation cannot
    /// be reserved.
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
        let mut accepted = Vec::new();
        if accepted.try_reserve_exact(N).is_err() {
            return Err(BytesConstructionRejected {
                bytes,
                reason: BoundedOwnerError::AllocationFailed,
            });
        }
        accepted.extend(bytes);
        Ok(Self { bytes: accepted })
    }

    /// Borrows the validated text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        std::str::from_utf8(&self.bytes).expect("bounded UTF-8 was validated at construction")
    }

    /// Borrows the validated UTF-8 bytes.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the exact UTF-8 byte length.
    #[must_use]
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Reports whether the text is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Consumes the owner and returns its validated bytes.
    #[must_use]
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }
}

/// Immutable bytes that can be shared without exposing mutable storage.
#[derive(Clone, Debug)]
pub struct OwnedBytes {
    bytes: ArcBytes,
}

impl OwnedBytes {
    /// Accepts bytes whose exact length does not exceed `max_bytes`.
    ///
    /// # Errors
    ///
    /// A length or allocation rejection returns the original byte allocation
    /// unchanged.
    pub fn try_from_vec(bytes: Vec<u8>, max_bytes: u64) -> Result<Self, BytesConstructionRejected> {
        Self::try_from_vec_with(bytes, max_bytes, |bytes| {
            ArcBytes::try_from_slice(bytes).map_err(|_| ())
        })
    }

    fn try_from_vec_with<E>(
        bytes: Vec<u8>,
        max_bytes: u64,
        allocate: impl FnOnce(&[u8]) -> Result<ArcBytes, E>,
    ) -> Result<Self, BytesConstructionRejected> {
        if u64::try_from(bytes.len()).expect("usize fits u64 on supported targets") > max_bytes {
            return Err(BytesConstructionRejected {
                bytes,
                reason: BoundedOwnerError::LengthExceedsCapacity,
            });
        }
        let shared = allocate(&bytes).map_err(|_| BytesConstructionRejected {
            bytes,
            reason: BoundedOwnerError::AllocationFailed,
        })?;
        Ok(Self { bytes: shared })
    }

    /// Borrows the immutable accepted bytes.
    #[must_use]
    pub fn as_slice(&self) -> &[u8] {
        &self.bytes
    }

    /// Returns the exact immutable byte length.
    #[must_use]
    pub fn len(&self) -> u64 {
        u64::try_from(self.bytes.len()).expect("usize fits u64 on supported targets")
    }

    /// Reports whether the immutable byte owner is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Consumes this owner and transfers its immutable shared allocation.
    #[must_use]
    pub fn into_arc(self) -> ArcBytes {
        self.bytes
    }
}

fn ensure_element_capacity<T>(capacity: usize) -> Result<(), BoundedOwnerError> {
    if std::mem::size_of::<T>() != 0 && capacity > isize::MAX as usize / std::mem::size_of::<T>() {
        return Err(BoundedOwnerError::CapacityTooLarge);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{BoundedOwnerError, OwnedBytes};

    #[test]
    fn owned_bytes_returns_the_original_vector_when_shared_allocation_fails() {
        let mut bytes = Vec::with_capacity(16);
        bytes.extend_from_slice(&[1, 2, 3]);
        let original_pointer = bytes.as_ptr();
        let original_capacity = bytes.capacity();

        let rejected = OwnedBytes::try_from_vec_with(bytes, 3, |_| Err(())).unwrap_err();

        assert_eq!(rejected.reason, BoundedOwnerError::AllocationFailed);
        assert_eq!(rejected.bytes, [1, 2, 3]);
        assert_eq!(rejected.bytes.as_ptr(), original_pointer);
        assert_eq!(rejected.bytes.capacity(), original_capacity);
    }
}
