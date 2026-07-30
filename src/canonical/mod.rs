//! Canonical values whose representations participate in stable contracts.

mod identity;

pub use identity::{
    BlobDigest, CanonicalHash, CanonicalOrder, ContentDigest, ContractDigest, DeviceGeneration,
    InputSourceId, MaterialId, NewtypeValueError, ParticipantId, ReceiptId, RngStreamId,
    SchemaDigest, Tick, VolumeId, VolumeIdRegistry, VolumeRegistryError, VolumeRevision, WorldId,
};
