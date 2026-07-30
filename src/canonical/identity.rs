//! Fixed-width stable identifiers, counters, hashes, and digests.
//!
//! The contained values are private so constrained identities can only be
//! created through their validating constructors.
//!
//! ```compile_fail
//! use moria::canonical::MaterialId;
//!
//! let _ = MaterialId(1);
//! ```

/// The reason a supplied newtype value cannot be represented.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NewtypeValueError {
    /// Zero is reserved and cannot identify an instance.
    ZeroReserved,
    /// The value lies outside the type's declared domain.
    OutOfRange,
    /// An all-zero byte value is reserved.
    AllZeroReserved,
}

macro_rules! constrained_scalar_id {
    ($name:ident, $raw:ty, $valid:expr, $doc:literal) => {
        #[doc = $doc]
        ///
        /// # Errors
        ///
        /// `try_from_raw` returns [`NewtypeValueError::ZeroReserved`] for zero
        /// and [`NewtypeValueError::OutOfRange`] for values outside this
        /// identity's declared range. It does not allocate or normalize input.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name($raw);

        impl $name {
            /// Validates and preserves one raw stable identity value.
            #[must_use = "the validated identity must be retained or inspected"]
            pub fn try_from_raw(raw: $raw) -> Result<Self, NewtypeValueError> {
                if raw == 0 {
                    Err(NewtypeValueError::ZeroReserved)
                } else if !($valid)(raw) {
                    Err(NewtypeValueError::OutOfRange)
                } else {
                    Ok(Self(raw))
                }
            }

            /// Returns the exact raw value accepted at construction.
            #[must_use]
            pub const fn get(self) -> $raw {
                self.0
            }
        }
    };
}

constrained_scalar_id!(
    MaterialId,
    u16,
    |_| true,
    "A consumer-defined nonzero material identity."
);
constrained_scalar_id!(
    VolumeId,
    u64,
    |_| true,
    "A nonzero stable identity for a logical volume."
);
constrained_scalar_id!(
    ParticipantId,
    u32,
    |raw| raw <= 0x7fff_ffff,
    "A nonzero participant identity in the shared ordering namespace."
);
constrained_scalar_id!(
    InputSourceId,
    u32,
    |raw| raw <= 0x7fff_ffff,
    "A nonzero input-source identity in the shared ordering namespace."
);
constrained_scalar_id!(
    RngStreamId,
    u32,
    |_| true,
    "A nonzero RNG stream identity scoped to one participant descriptor."
);

/// A consumer-supplied 128-bit world identity.
///
/// Every byte sequence is valid and the byte accessors preserve every bit.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct WorldId([u8; 16]);

impl WorldId {
    /// Creates an identity from its exact 16-byte representation.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 16]) -> Self {
        Self(bytes)
    }

    /// Borrows the exact 16-byte representation.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.0
    }

    /// Returns the exact 16-byte representation.
    #[must_use]
    pub const fn to_bytes(self) -> [u8; 16] {
        self.0
    }
}

macro_rules! counter_value {
    ($name:ident, $raw:ty, $doc:literal) => {
        #[doc = $doc]
        ///
        /// Zero and the maximum raw value are both valid. Counter allocation
        /// and exhaustion policy belong to the owning canonical registry.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name($raw);

        impl $name {
            /// Preserves one raw counter value without allocation or validation.
            #[must_use]
            pub const fn from_raw(raw: $raw) -> Self {
                Self(raw)
            }

            /// Returns the exact raw counter value.
            #[must_use]
            pub const fn get(self) -> $raw {
                self.0
            }
        }
    };
}

counter_value!(Tick, u64, "A sealed canonical tick number.");
counter_value!(VolumeRevision, u64, "A canonical volume revision number.");
counter_value!(
    CanonicalOrder,
    u32,
    "A canonical command order within one tick."
);
counter_value!(
    DeviceGeneration,
    u64,
    "A noncanonical device-recovery generation."
);
counter_value!(
    ReceiptId,
    u64,
    "A noncanonical accepted-operation receipt identity."
);

macro_rules! digest {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        ///
        /// Every 32-byte value is valid, including all zeroes. Byte accessors
        /// preserve every bit and never allocate.
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name([u8; 32]);

        impl $name {
            /// Creates this digest from its exact 32-byte representation.
            #[must_use]
            pub const fn from_bytes(bytes: [u8; 32]) -> Self {
                Self(bytes)
            }

            /// Borrows the exact 32-byte representation.
            #[must_use]
            pub const fn as_bytes(&self) -> &[u8; 32] {
                &self.0
            }

            /// Returns the exact 32-byte representation.
            #[must_use]
            pub const fn to_bytes(self) -> [u8; 32] {
                self.0
            }
        }
    };
}

digest!(CanonicalHash, "A canonical-state hash.");
digest!(ContentDigest, "An immutable content digest.");
digest!(ContractDigest, "A canonical-contract digest.");
digest!(SchemaDigest, "A schema digest.");
digest!(BlobDigest, "An immutable blob digest.");
