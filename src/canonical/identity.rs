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
//!
//! ```compile_fail
//! use moria::canonical::VolumeId;
//!
//! let _ = VolumeId(1);
//! ```
//!
//! ```compile_fail
//! use moria::canonical::ParticipantId;
//!
//! let _ = ParticipantId(1);
//! ```
//!
//! ```compile_fail
//! use moria::canonical::InputSourceId;
//!
//! let _ = InputSourceId(1);
//! ```
//!
//! ```compile_fail
//! use moria::canonical::RngStreamId;
//!
//! let _ = RngStreamId(1);
//! ```
//!
//! ```compile_fail
//! use moria::canonical::MaterialId;
//!
//! let _ = MaterialId::from_raw(1);
//! ```
//!
//! ```compile_fail
//! use moria::canonical::{MaterialId, VolumeId};
//!
//! let material = MaterialId::try_from_raw(1).unwrap();
//! let _: VolumeId = material;
//! ```
//!
//! ```compile_fail
//! use moria::canonical::{CanonicalHash, ContentDigest};
//!
//! let hash = CanonicalHash::from_bytes([0; 32]);
//! let _: ContentDigest = hash;
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
        #[doc = concat!(
                            $doc,
                            "\n\n# Errors\n\n`try_from_raw` returns ",
                            "[`NewtypeValueError::ZeroReserved`] for zero and ",
                            "[`NewtypeValueError::OutOfRange`] for values outside this identity's ",
                            "declared range. It does not allocate or normalize input.\n\n",
                            "# Examples\n\n```\nuse moria::canonical::",
                            stringify!($name),
                            ";\n\nlet id = ",
                            stringify!($name),
                            "::try_from_raw(1).unwrap();\nassert_eq!(id.get(), 1);\n```"
                        )]
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
    ///
    /// # Examples
    ///
    /// ```
    /// use moria::canonical::WorldId;
    ///
    /// let id = WorldId::from_bytes([0x80; 16]);
    /// assert_eq!(id.as_bytes(), &[0x80; 16]);
    /// assert_eq!(id.to_bytes(), [0x80; 16]);
    /// ```
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
        #[doc = concat!(
                    $doc,
                    "\n\nZero and the maximum raw value are both valid. Counter allocation ",
                    "and exhaustion policy belong to the owning canonical registry.\n\n",
                    "# Examples\n\n```\nuse moria::canonical::",
                    stringify!($name),
                    ";\n\nassert_eq!(",
                    stringify!($name),
                    "::from_raw(0).get(), 0);\n```"
                )]
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
        #[doc = concat!(
                    $doc,
                    "\n\nEvery 32-byte value is valid, including all zeroes. Byte accessors ",
                    "preserve every bit and never allocate.\n\n",
                    "# Examples\n\n```\nuse moria::canonical::",
                    stringify!($name),
                    ";\n\nlet digest = ",
                    stringify!($name),
                    "::from_bytes([0x55; 32]);\nassert_eq!(digest.as_bytes(), &[0x55; 32]);\n",
                    "assert_eq!(digest.to_bytes(), [0x55; 32]);\n```"
                )]
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

#[cfg(test)]
/// Errors emitted while assigning the fixed canonical volume serials.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum VolumeRegistryError {
    DuplicateGenesisId(VolumeId),
    Exhausted,
}

#[cfg(test)]
/// Bounded serial allocation state used while validating a world's volume registry.
///
/// This remains internal until the world-configuration facade owns registry
/// construction. It nevertheless keeps the TECH-005 uniqueness and exhaustion
/// rules in one canonical implementation rather than reproducing them at each
/// caller.
#[derive(Clone, Debug, Eq, PartialEq)]
struct VolumeIdRegistry {
    claimed: std::collections::BTreeSet<VolumeId>,
    capacity: usize,
    next_volume_serial: Option<VolumeId>,
}

#[cfg(test)]
impl VolumeIdRegistry {
    fn from_genesis(capacity: usize, genesis: &[VolumeId]) -> Result<Self, VolumeRegistryError> {
        let mut claimed = std::collections::BTreeSet::new();
        for &id in genesis {
            if !claimed.insert(id) {
                return Err(VolumeRegistryError::DuplicateGenesisId(id));
            }
        }
        if claimed.len() > capacity {
            return Err(VolumeRegistryError::Exhausted);
        }

        let next_volume_serial = match claimed.last() {
            Some(id) => id
                .get()
                .checked_add(1)
                .map(VolumeId::try_from_raw)
                .transpose()
                .expect("a nonzero successor must remain a valid volume identity"),
            None => {
                Some(VolumeId::try_from_raw(1).expect("one is always a valid first volume serial"))
            }
        };

        Ok(Self {
            claimed,
            capacity,
            next_volume_serial,
        })
    }

    fn next_volume_serial(&self) -> Option<VolumeId> {
        self.next_volume_serial
    }

    fn allocate_next(&mut self) -> Result<VolumeId, VolumeRegistryError> {
        if self.claimed.len() == self.capacity {
            return Err(VolumeRegistryError::Exhausted);
        }
        let id = self
            .next_volume_serial
            .ok_or(VolumeRegistryError::Exhausted)?;
        let next_volume_serial = id
            .get()
            .checked_add(1)
            .map(VolumeId::try_from_raw)
            .transpose()
            .expect("a nonzero successor must remain a valid volume identity");

        let inserted = self.claimed.insert(id);
        debug_assert!(inserted, "the next serial follows the greatest claimed ID");
        self.next_volume_serial = next_volume_serial;
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::{VolumeId, VolumeIdRegistry, VolumeRegistryError};

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
}
