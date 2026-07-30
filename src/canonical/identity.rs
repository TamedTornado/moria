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

/// Errors emitted while assigning or resolving canonical volume identities.
#[allow(
    dead_code,
    reason = "the sealed-tick world owner that consumes this internal error is introduced after TECH-005"
)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum VolumeRegistryError {
    /// Genesis claimed the same stable volume ID more than once.
    DuplicateGenesisId(VolumeId),
    /// The registry has no remaining capacity or no representable next serial.
    Exhausted,
    /// The requested ID has never been claimed by this world.
    AbsentReference(VolumeId),
    /// The requested ID is no longer live in this world.
    RetiredReference(VolumeId),
}

/// Bounded serial allocation state used while validating a world's volume registry.
///
/// Genesis IDs must be unique. The next serial is one above the greatest
/// genesis ID, and IDs are never reused. `capacity` bounds the total number of
/// claimed IDs for the lifetime of this registry. It is crate-internal until
/// the sealed-tick world owner can enforce allocation and retirement ordering.
#[allow(
    dead_code,
    reason = "the sealed-tick world owner that owns volume allocation is introduced after TECH-005"
)]
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct VolumeIdRegistry {
    claimed: std::collections::BTreeSet<VolumeId>,
    retired: std::collections::BTreeSet<VolumeId>,
    capacity: usize,
    next_volume_serial: Option<VolumeId>,
}

#[allow(
    dead_code,
    reason = "the sealed-tick world owner invokes these internal operations after TECH-005"
)]
impl VolumeIdRegistry {
    /// Creates a bounded registry from the IDs claimed at world genesis.
    ///
    /// The supplied IDs remain claimed for the registry lifetime. See
    /// [`Self`] for the uniqueness and capacity error contract.
    pub(crate) fn from_genesis(
        capacity: usize,
        genesis: &[VolumeId],
    ) -> Result<Self, VolumeRegistryError> {
        if genesis.len() > capacity {
            return Err(VolumeRegistryError::Exhausted);
        }

        let mut claimed = std::collections::BTreeSet::new();
        for &id in genesis {
            if !claimed.insert(id) {
                return Err(VolumeRegistryError::DuplicateGenesisId(id));
            }
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
            retired: std::collections::BTreeSet::new(),
            capacity,
            next_volume_serial,
        })
    }

    /// Returns the next unclaimed serial, or `None` when `u64` serial space is exhausted.
    ///
    /// This inspection never mutates the registry. Capacity exhaustion is
    /// reported only by [`Self::allocate_next`].
    pub(crate) fn next_volume_serial(&self) -> Option<VolumeId> {
        self.next_volume_serial
    }

    /// Claims and returns the next serial during sealed-tick processing.
    pub(crate) fn allocate_next(&mut self) -> Result<VolumeId, VolumeRegistryError> {
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

    /// Resolves an ID that a canonical command references.
    pub(crate) fn validate_live_reference(&self, id: VolumeId) -> Result<(), VolumeRegistryError> {
        if self.retired.contains(&id) {
            Err(VolumeRegistryError::RetiredReference(id))
        } else if self.claimed.contains(&id) {
            Ok(())
        } else {
            Err(VolumeRegistryError::AbsentReference(id))
        }
    }

    /// Marks a live ID retired after its sealed canonical transition.
    pub(crate) fn retire(&mut self, id: VolumeId) -> Result<(), VolumeRegistryError> {
        self.validate_live_reference(id)?;
        let inserted = self.retired.insert(id);
        debug_assert!(inserted, "a validated live ID cannot already be retired");
        Ok(())
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
    fn oversized_genesis_is_rejected_before_duplicate_validation_or_set_allocation() {
        let duplicate = VolumeId::try_from_raw(7).unwrap();
        assert_eq!(
            VolumeIdRegistry::from_genesis(1, &[duplicate, duplicate]),
            Err(VolumeRegistryError::Exhausted)
        );
    }

    #[test]
    fn bounded_genesis_rejects_duplicate_ids() {
        let duplicate = VolumeId::try_from_raw(7).unwrap();
        assert_eq!(
            VolumeIdRegistry::from_genesis(2, &[duplicate, duplicate]),
            Err(VolumeRegistryError::DuplicateGenesisId(duplicate))
        );
    }

    #[test]
    fn volume_references_report_absent_and_retired_ids_without_mutation() {
        let active = VolumeId::try_from_raw(4).unwrap();
        let absent = VolumeId::try_from_raw(5).unwrap();
        let mut registry = VolumeIdRegistry::from_genesis(2, &[active]).unwrap();

        assert_eq!(
            registry.validate_live_reference(absent),
            Err(VolumeRegistryError::AbsentReference(absent))
        );
        let before = registry.clone();
        assert_eq!(
            registry.retire(absent),
            Err(VolumeRegistryError::AbsentReference(absent))
        );
        assert_eq!(registry, before);

        assert_eq!(registry.retire(active), Ok(()));
        let before = registry.clone();
        assert_eq!(
            registry.validate_live_reference(active),
            Err(VolumeRegistryError::RetiredReference(active))
        );
        assert_eq!(registry, before);
        assert_eq!(
            registry.retire(active),
            Err(VolumeRegistryError::RetiredReference(active))
        );
        assert_eq!(registry, before);
    }
}
