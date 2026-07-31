//! Closed, actionable public failure records.

use crate::canonical::{
    BlobDigest, CanonicalHash, DeviceGeneration, InputSourceId, NewtypeValueError, ParticipantId,
    ReceiptId, Tick, VolumeId, WorldId,
};

use super::{BoundedOwnerError, BoundedUtf8, BoundedVec};

macro_rules! provider_id {
    ($name:ident, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        pub struct $name(u32);

        impl $name {
            /// Validates one stable provider identity.
            ///
            /// # Errors
            ///
            /// Returns [`NewtypeValueError::ZeroReserved`] for zero and
            /// [`NewtypeValueError::OutOfRange`] above `0x7fff_ffff`.
            pub fn try_from_raw(raw: u32) -> Result<Self, NewtypeValueError> {
                if raw == 0 {
                    Err(NewtypeValueError::ZeroReserved)
                } else if raw > 0x7fff_ffff {
                    Err(NewtypeValueError::OutOfRange)
                } else {
                    Ok(Self(raw))
                }
            }

            /// Returns the validated stable identity value.
            #[must_use]
            pub const fn get(self) -> u32 {
                self.0
            }
        }
    };
}

provider_id!(
    BaseContentSourceId,
    "A stable base-content source identity."
);
provider_id!(BaseAuthorityId, "A stable base-authority identity.");
provider_id!(ContentBlobStoreId, "A stable content-blob store identity.");
provider_id!(CheckpointStoreId, "A stable checkpoint-store identity.");
provider_id!(ReplaySinkId, "A stable replay-sink identity.");

/// A closed stable failure classification.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[allow(clippy::enum_variant_names)]
pub enum ErrorCode {
    InvalidConfig,
    DuplicateId,
    MissingId,
    WrongProviderKind,
    ContractMismatch,
    UnsupportedVersion,
    InvalidBounds,
    InvalidEncoding,
    InvalidOrientation,
    InvalidCell,
    WrongWorld,
    WorldUnknown,
    BeforeNextTick,
    AfterNextTick,
    AlreadyPending,
    WorldNotReady,
    WorldClosed,
    WorldFailed,
    TelemetryBusy,
    AlreadyShuttingDown,
    DependencyNotReady,
    QueueFull,
    CapacityExceeded,
    CanonicalBudget,
    PersistenceBackpressure,
    StaleRevision,
    StaleHash,
    ArithmeticOverflow,
    SourceUnavailable,
    SourceInvalid,
    ProducerDropped,
    StoreFailure,
    ManifestNotFound,
    UnsupportedAtomicCommit,
    CorruptBlob,
    LineageMismatch,
    FrontierUnavailable,
    FrontierTooOld,
    ResultCapacityExceeded,
    ObservationGap,
    ParticipantFailure,
    ParticipantDivergence,
    ReplayDivergence,
    BackendUnavailable,
    DeterminismViolation,
    DeviceLost,
    MappingFailure,
    DecodeFailure,
    Cancelled,
    Shutdown,
    InternalInvariant,
}

/// The affected durable or operation boundary.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FailureScope {
    Configuration,
    World(WorldId),
    Tick { world: WorldId, tick: Tick },
    Volume { world: WorldId, volume: VolumeId },
    Operation(ReceiptId),
    Provider(ProviderId),
}

/// A frozen provider identity used in a failure scope.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ProviderId {
    InputSource(InputSourceId),
    BaseSource(BaseContentSourceId),
    BaseAuthority(BaseAuthorityId),
    ContentBlobStore(ContentBlobStoreId),
    CheckpointStore(CheckpointStoreId),
    ReplaySink(ReplaySinkId),
    Participant(ParticipantId),
}

/// The resource-budget group containing a stable field ordinal.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BudgetGroup {
    Identity,
    Canonical,
    Content,
    Query,
    Observation,
    Presentation,
    Checkpoint,
    Rollback,
    Participant,
    Runtime,
}

/// A stable budget-field locator.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ResourceBudgetField {
    pub group: BudgetGroup,
    pub field_code: u16,
}

/// Whether retry requires a new request, a dependency, recovery, or is impossible.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Retryability {
    RetryNewRequest,
    RetryAfterDependency,
    RetryAfterRecovery,
    Never,
}

/// The authoritative effect already committed by the failed operation.
#[derive(Debug, Eq, PartialEq)]
pub enum CommittedEffect {
    None,
    Frontier(FrontierSummary),
}

/// A closed frontier position; genesis has no sentinel tick.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum FrontierPosition {
    Genesis,
    Confirmed(Tick),
}

impl FrontierPosition {
    /// Returns the checked next eligible tick, if one exists.
    #[must_use]
    pub const fn next_tick(self) -> Option<Tick> {
        match self {
            Self::Genesis => Some(Tick::from_raw(0)),
            Self::Confirmed(tick) => match tick.get().checked_add(1) {
                Some(next) => Some(Tick::from_raw(next)),
                None => None,
            },
        }
    }
}

/// The authority quality of a copied frontier summary.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AuthorityStatus {
    ReplayGrade,
    DiagnosticCandidate,
}

/// The complete public identity of an installed frontier.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct FrontierSummary {
    pub world: WorldId,
    pub position: FrontierPosition,
    pub root_hash: CanonicalHash,
    pub status: AuthorityStatus,
}

impl FrontierSummary {
    /// Returns the checked next eligible tick for this frontier.
    #[must_use]
    pub const fn next_tick(&self) -> Option<Tick> {
        self.position.next_tick()
    }
}

/// An ordinary operation failure with its scope and committed-effect fact.
#[derive(Debug, Eq, PartialEq)]
pub struct OperationError {
    pub code: ErrorCode,
    pub scope: FailureScope,
    pub retryability: Retryability,
    pub committed: CommittedEffect,
    pub diagnostic: BoundedUtf8<160>,
}

impl OperationError {
    /// Creates an operation failure with a bounded diagnostic.
    ///
    /// # Errors
    ///
    /// Rejects diagnostics that are not valid UTF-8 or exceed 160 bytes.
    pub fn new(
        code: ErrorCode,
        scope: FailureScope,
        retryability: Retryability,
        committed: CommittedEffect,
        diagnostic: &str,
    ) -> Result<Self, FailureRecordError> {
        let text = BoundedUtf8::try_from_bytes(diagnostic.as_bytes().to_vec())
            .map_err(|rejected| FailureRecordError::Diagnostic(rejected.reason))?;
        Ok(Self {
            code,
            scope,
            retryability,
            committed,
            diagnostic: text,
        })
    }
}

/// The validation failure for a public failure-record constructor.
#[derive(Debug, Eq, PartialEq)]
pub enum FailureRecordError {
    Diagnostic(BoundedOwnerError),
    AdmissionContextMismatch,
    NoAdvanceSourceFrontier,
    NoAdvanceScope,
    NoAdvanceCauseCode,
    NoAdvanceCommittedEffect,
    CorrectionCommittedEffect,
    CorrectionExportFailure,
}

/// A nonzero consumer-selected replay stream key.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ReplayStreamKey([u8; 32]);

impl ReplayStreamKey {
    /// Validates and preserves a replay stream key.
    ///
    /// # Errors
    ///
    /// Rejects the all-zero reserved key without normalizing any other byte.
    pub fn try_from_bytes(bytes: [u8; 32]) -> Result<Self, NewtypeValueError> {
        if bytes == [0; 32] {
            return Err(NewtypeValueError::AllZeroReserved);
        }
        Ok(Self(bytes))
    }

    /// Borrows the exact accepted bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Returns the exact accepted bytes.
    #[must_use]
    pub const fn to_bytes(self) -> [u8; 32] {
        self.0
    }
}

/// The append range a replay-sink request attempted to store.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ReplayAppendRange {
    Header {
        starting: FrontierPosition,
        next_tick: Tick,
    },
    TickRecords {
        first_tick: Tick,
        last_tick: Tick,
        record_count: u32,
    },
    CorrectionBranch {
        target_tick: Tick,
        superseded_through: Tick,
        corrected_through: Tick,
        record_count: u32,
    },
}

/// A replay request retained when its append failed.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ReplaySinkRequest {
    pub stream: ReplayStreamKey,
    pub sequence: u64,
    pub range: ReplayAppendRange,
    pub bytes: u64,
    pub digest: BlobDigest,
}

/// Exact replay append failure context retained by a correction failure.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ReplayExportFailure {
    pub sink: ReplaySinkId,
    pub request: ReplaySinkRequest,
    pub failure: ErrorCode,
}

/// A correction failure retaining the unchanged live frontier.
#[derive(Debug, Eq, PartialEq)]
pub struct CorrectionError {
    pub original_frontier: FrontierSummary,
    pub error: OperationError,
    pub replay_export_failure: Option<ReplayExportFailure>,
}

impl CorrectionError {
    /// Creates a correction failure only when correction publication did not occur.
    ///
    /// # Errors
    ///
    /// Rejects a committed effect and invalid correction-branch append failure facts.
    pub fn try_new(
        original_frontier: FrontierSummary,
        error: OperationError,
        replay_export_failure: Option<ReplayExportFailure>,
    ) -> Result<Self, FailureRecordError> {
        if error.committed != CommittedEffect::None {
            return Err(FailureRecordError::CorrectionCommittedEffect);
        }
        if let Some(export) = replay_export_failure
            && (error.code != ErrorCode::StoreFailure
                || error.scope != FailureScope::Provider(ProviderId::ReplaySink(export.sink))
                || error.retryability != Retryability::Never
                || export.failure != ErrorCode::StoreFailure)
        {
            return Err(FailureRecordError::CorrectionExportFailure);
        }
        Ok(Self {
            original_frontier,
            error,
            replay_export_failure,
        })
    }
}

/// The stable reason a sealed tick did not advance.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TickNoAdvanceCause {
    Canonical(CanonicalFailure),
    Participant {
        participant: ParticipantId,
        code: ErrorCode,
    },
    Provider {
        provider: ProviderId,
        code: ErrorCode,
    },
    Device {
        generation: DeviceGeneration,
        code: ErrorCode,
    },
    Shutdown,
    Internal(ErrorCode),
}

impl TickNoAdvanceCause {
    fn matches(self, code: ErrorCode) -> bool {
        match self {
            Self::Canonical(_) => true,
            Self::Participant { code: cause, .. }
            | Self::Provider { code: cause, .. }
            | Self::Device { code: cause, .. }
            | Self::Internal(cause) => cause == code,
            Self::Shutdown => code == ErrorCode::Shutdown,
        }
    }
}

/// A closed canonical transition failure reason.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CanonicalFailure {
    MissingIdentity,
    WrongVolumeKind,
    StaleRevision,
    StaleSourceHash,
    InvalidBounds,
    InvalidCell,
    InvalidOrientation,
    InvalidFixedFormat,
    ArithmeticOverflow,
    DivisionByZero,
    InvalidShift,
    NegativeSquareRoot,
    Nonrepresentable,
    LogicalCapacity,
    DependencyUnavailable,
    ParticipantEffectInvalid,
    ParticipantFailed,
    InjectedCandidateFailure,
    ZeroAxis,
    UnrepresentableAxis,
}

/// A failed sealed tick that preserved the source frontier.
#[derive(Debug, Eq, PartialEq)]
pub struct FailedNoAdvance {
    pub world: WorldId,
    pub attempted_tick: Tick,
    pub source_frontier: FrontierSummary,
    pub cause: TickNoAdvanceCause,
    pub error: OperationError,
}

impl FailedNoAdvance {
    /// Creates a tick failure after checking its preserved-frontier invariants.
    ///
    /// # Errors
    ///
    /// Rejects any mismatched source tick, scope, cause/error class, or committed effect.
    pub fn try_new(
        world: WorldId,
        attempted_tick: Tick,
        source_frontier: FrontierSummary,
        cause: TickNoAdvanceCause,
        error: OperationError,
    ) -> Result<Self, FailureRecordError> {
        if source_frontier.world != world || source_frontier.next_tick() != Some(attempted_tick) {
            return Err(FailureRecordError::NoAdvanceSourceFrontier);
        }
        if error.scope
            != (FailureScope::Tick {
                world,
                tick: attempted_tick,
            })
        {
            return Err(FailureRecordError::NoAdvanceScope);
        }
        if error.committed != CommittedEffect::None {
            return Err(FailureRecordError::NoAdvanceCommittedEffect);
        }
        if !cause.matches(error.code) {
            return Err(FailureRecordError::NoAdvanceCauseCode);
        }
        Ok(Self {
            world,
            attempted_tick,
            source_frontier,
            cause,
            error,
        })
    }
}

/// Synchronous telemetry failures, each deliberately fieldless beyond its facts.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum TelemetryError {
    WorldUnknown {
        world: WorldId,
    },
    WorldClosed {
        world: WorldId,
        last_frontier: Option<FrontierSummary>,
    },
    TelemetryBusy {
        world: WorldId,
    },
}

/// Capacity derived before or after query execution.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct QueryCapacity {
    pub bricks: u64,
    pub records: u64,
    pub result_bytes: u64,
    pub workgroups: u64,
    pub volume_revisions: u64,
}

/// Capacity derived for an interest request.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct InterestCapacity {
    pub bricks: u64,
}

/// The reason a query's requested range is unavailable.
#[derive(Debug)]
pub struct MissingRange {
    pub volume: VolumeId,
    pub reason: AvailabilityCode,
}

/// A closed availability classification for an unavailable range.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AvailabilityCode {
    Cold,
    Materializing,
    Failed,
    FrontierTooOld,
    DeviceLost,
    CapacityExceeded,
}

/// A terminal query outcome that never substitutes unavailable matter for empty matter.
#[derive(Debug)]
pub enum QueryUnavailable {
    Availability {
        error: OperationError,
        missing: BoundedVec<MissingRange>,
    },
    ResultCapacityExceeded {
        required: QueryCapacity,
        supported: QueryCapacity,
    },
}

/// An admission failure with an exactly matching context.
#[derive(Debug, Eq, PartialEq)]
pub struct AdmissionError {
    pub code: AdmissionCode,
    pub retryability: Retryability,
    pub context: AdmissionContext,
}

impl AdmissionError {
    /// Creates an admission error only for the context legal for `code`.
    ///
    /// # Errors
    ///
    /// Rejects context/code pairs that TECH-027 forbids from being emitted.
    pub fn try_new(
        code: AdmissionCode,
        retryability: Retryability,
        context: AdmissionContext,
    ) -> Result<Self, FailureRecordError> {
        if !context.matches(code) {
            return Err(FailureRecordError::AdmissionContextMismatch);
        }
        Ok(Self {
            code,
            retryability,
            context,
        })
    }
}

/// The exact fact accompanying an admission error.
#[derive(Debug, Eq, PartialEq)]
pub enum AdmissionContext {
    None,
    TickEligibility {
        supplied: Tick,
        expected_next: Tick,
    },
    InvalidBatch {
        reason: BatchError,
    },
    InterestCapacity {
        required: InterestCapacity,
        supported: InterestCapacity,
    },
    QueryCapacity {
        required: QueryCapacity,
        supported: QueryCapacity,
    },
    CorrectionExpectedHashCount {
        replacement_batches: u32,
        expected_hashes: u32,
    },
    BudgetCapacity {
        field: ResourceBudgetField,
        required: u64,
        supported: u64,
    },
}

impl AdmissionContext {
    const fn matches(&self, code: AdmissionCode) -> bool {
        matches!(
            (code, self),
            (
                AdmissionCode::BeforeNextTick | AdmissionCode::AfterNextTick,
                Self::TickEligibility { .. }
            ) | (AdmissionCode::InvalidBatch, Self::InvalidBatch { .. })
                | (
                    AdmissionCode::InterestTooLarge,
                    Self::InterestCapacity { .. }
                )
                | (
                    AdmissionCode::ResultCapacityExceeded,
                    Self::QueryCapacity { .. }
                )
                | (
                    AdmissionCode::CorrectionHashCountMismatch,
                    Self::CorrectionExpectedHashCount { .. }
                )
                | (
                    AdmissionCode::RetiredReplayStreamCapacity,
                    Self::BudgetCapacity { .. }
                )
        ) || (matches!(self, Self::None)
            && !matches!(
                code,
                AdmissionCode::BeforeNextTick
                    | AdmissionCode::AfterNextTick
                    | AdmissionCode::InvalidBatch
                    | AdmissionCode::InterestTooLarge
                    | AdmissionCode::ResultCapacityExceeded
                    | AdmissionCode::CorrectionHashCountMismatch
                    | AdmissionCode::RetiredReplayStreamCapacity
            ))
    }
}

/// A closed pre-admission rejection classification.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum AdmissionCode {
    WrongWorld,
    WrongState,
    DuplicateReplayStream,
    RetiredReplayStreamCapacity,
    BeforeNextTick,
    AfterNextTick,
    AlreadyPending,
    WorldNotReady,
    DependencyNotReady,
    Full,
    Closed,
    InvalidRequest,
    InvalidBatch,
    InterestTooLarge,
    ResultCapacityExceeded,
    CorrectionHashCountMismatch,
    StaleGeneration,
    PersistenceBackpressure,
}

/// A closed batch validation reason used by admission context.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BatchError {
    Empty,
    CountMismatch,
    DuplicateCanonicalKey,
    EncodingFailure,
    ReservationMismatch,
}

/// The top-level closed public error taxonomy.
#[derive(Debug)]
pub enum MoriaError {
    Config(ConfigError),
    Admission(AdmissionError),
    Operation(OperationError),
    Tick(FailedNoAdvance),
    Query(QueryUnavailable),
    Telemetry(TelemetryError),
}

/// Configuration validation failure retained as a closed record.
#[derive(Debug, Eq, PartialEq)]
pub struct ConfigError {
    pub code: ConfigErrorCode,
    pub field: ConfigField,
    pub diagnostic: BoundedUtf8<160>,
}

/// The closed configuration failure class.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ConfigErrorCode {
    DuplicateId,
    RetiredReplayStreamCapacity,
    MissingReference,
    WrongProviderKind,
    ContractMismatch,
    InvalidValue,
    CrossLimitViolation,
    UnsupportedCapability,
    ArithmeticOverflow,
}

/// The configuration field carrying the failure.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum ConfigField {
    Canonical,
    Budgets(ResourceBudgetField),
    Rollback,
    Persistence,
    Presentation,
    Execution,
    Material,
    InputSource,
    BaseSource,
    BaseAuthority,
    ContentBlobStore,
    CheckpointStore,
    ReplaySink,
    Volume,
    Participant,
}
