//! Closed, actionable public failure records.

use crate::canonical::{
    BlobDigest, CanonicalHash, DeviceGeneration, InputSourceId, LocalCellAabb, NewtypeValueError,
    ParticipantId, ReceiptId, Tick, VolumeId, WorldId,
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

impl ErrorCode {
    const fn is_replay_export_failure(self) -> bool {
        matches!(
            self,
            Self::StoreFailure | Self::ProducerDropped | Self::ContractMismatch
        )
    }
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

impl BudgetGroup {
    const fn field_count(self) -> u16 {
        match self {
            Self::Identity => 19,
            Self::Canonical => 8,
            Self::Content => 9,
            Self::Query => 7,
            Self::Observation => 10,
            Self::Presentation => 8,
            Self::Checkpoint => 10,
            Self::Rollback => 18,
            Self::Participant => 11,
            Self::Runtime => 4,
        }
    }

    /// Decodes a stable resource-budget group tag.
    ///
    /// # Examples
    ///
    /// ```
    /// use moria::facade::BudgetGroup;
    ///
    /// assert_eq!(BudgetGroup::try_from_wire_tag(0)?, BudgetGroup::Identity);
    /// # Ok::<(), moria::facade::ResourceBudgetFieldError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ResourceBudgetFieldError::UnknownGroupTag`] when `tag` is not
    /// a declared v1 resource-budget group.
    pub const fn try_from_wire_tag(tag: u8) -> Result<Self, ResourceBudgetFieldError> {
        match tag {
            0 => Ok(Self::Identity),
            1 => Ok(Self::Canonical),
            2 => Ok(Self::Content),
            3 => Ok(Self::Query),
            4 => Ok(Self::Observation),
            5 => Ok(Self::Presentation),
            6 => Ok(Self::Checkpoint),
            7 => Ok(Self::Rollback),
            8 => Ok(Self::Participant),
            9 => Ok(Self::Runtime),
            _ => Err(ResourceBudgetFieldError::UnknownGroupTag(tag)),
        }
    }
}

/// The validation failure for a resource-budget field wire locator.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResourceBudgetFieldError {
    /// The resource-budget group tag is not declared in this wire version.
    UnknownGroupTag(u8),
    /// The field ordinal is not declared within its resource-budget group.
    UnknownFieldOrdinal { group: BudgetGroup, field_code: u16 },
}

/// A stable budget-field locator.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ResourceBudgetField {
    pub group: BudgetGroup,
    pub field_code: u16,
}

impl ResourceBudgetField {
    /// Validates a stable resource-budget field locator.
    ///
    /// # Examples
    ///
    /// ```
    /// use moria::facade::{BudgetGroup, ResourceBudgetField};
    ///
    /// let field = ResourceBudgetField::try_new(BudgetGroup::Identity, 1)?;
    /// assert_eq!(field.field_code, 1);
    /// # Ok::<(), moria::facade::ResourceBudgetFieldError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ResourceBudgetFieldError::UnknownFieldOrdinal`] when
    /// `field_code` is not declared for `group` in this wire version.
    pub const fn try_new(
        group: BudgetGroup,
        field_code: u16,
    ) -> Result<Self, ResourceBudgetFieldError> {
        if field_code == 0 || field_code > group.field_count() {
            return Err(ResourceBudgetFieldError::UnknownFieldOrdinal { group, field_code });
        }
        Ok(Self { group, field_code })
    }

    /// Decodes a stable resource-budget field locator from its wire parts.
    ///
    /// # Examples
    ///
    /// ```
    /// use moria::facade::{BudgetGroup, ResourceBudgetField};
    ///
    /// let field = ResourceBudgetField::try_from_wire_parts(0, 1)?;
    /// assert_eq!(field.group, BudgetGroup::Identity);
    /// # Ok::<(), moria::facade::ResourceBudgetFieldError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`ResourceBudgetFieldError::UnknownGroupTag`] for an undeclared
    /// `group_tag`, or [`ResourceBudgetFieldError::UnknownFieldOrdinal`] for
    /// an undeclared ordinal in a known group.
    pub const fn try_from_wire_parts(
        group_tag: u8,
        field_code: u16,
    ) -> Result<Self, ResourceBudgetFieldError> {
        match BudgetGroup::try_from_wire_tag(group_tag) {
            Ok(group) => Self::try_new(group, field_code),
            Err(error) => Err(error),
        }
    }
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
        let text = BoundedUtf8::try_from_str(diagnostic).map_err(FailureRecordError::Diagnostic)?;
        Ok(Self {
            code,
            scope,
            retryability,
            committed,
            diagnostic: text,
        })
    }
}

/// The terminal error returned by genesis receipt polling.
pub type GenesisError = OperationError;

/// The terminal error returned by interest receipt polling.
pub type InterestError = OperationError;

/// The terminal error returned by observation snapshot receipt polling.
pub type ObservationSnapshotError = OperationError;

/// The terminal error returned by checkpoint receipt polling.
pub type CheckpointError = OperationError;

/// The terminal error returned by restore receipt polling.
pub type RestoreError = OperationError;

/// The terminal error returned by public replay receipt polling.
pub type ReplayFailure = OperationError;

/// The terminal error returned by participant receipt polling.
pub type ParticipantError = OperationError;

/// The terminal error returned by recovery receipt polling.
pub type RecoveryError = OperationError;

/// The terminal error returned by shutdown receipt polling.
pub type ShutdownError = OperationError;

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
        if error.code == ErrorCode::StoreFailure
            && matches!(
                error.scope,
                FailureScope::Provider(ProviderId::ReplaySink(_))
            )
            && replay_export_failure.is_none()
        {
            return Err(FailureRecordError::CorrectionExportFailure);
        }
        if let Some(export) = replay_export_failure
            && (error.code != ErrorCode::StoreFailure
                || error.scope != FailureScope::Provider(ProviderId::ReplaySink(export.sink))
                || error.retryability != Retryability::Never
                || !export.failure.is_replay_export_failure()
                || !correction_branch_append_matches_frontier(export.request, original_frontier))
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

fn correction_branch_append_matches_frontier(
    request: ReplaySinkRequest,
    original_frontier: FrontierSummary,
) -> bool {
    let FrontierPosition::Confirmed(original_live_tick) = original_frontier.position else {
        return false;
    };
    let ReplayAppendRange::CorrectionBranch {
        target_tick,
        superseded_through,
        corrected_through,
        record_count,
    } = request.range
    else {
        return false;
    };

    if request.sequence == 0
        || target_tick >= superseded_through
        || superseded_through != corrected_through
        || corrected_through != original_live_tick
    {
        return false;
    }

    match corrected_through.get().checked_sub(target_tick.get()) {
        Some(expected_count) if expected_count <= u64::from(u32::MAX) => {
            record_count == expected_count as u32
        }
        _ => false,
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
            Self::Canonical(cause) => cause.error_code() == code,
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

/// A failure decoding a stable canonical-failure wire tag.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum CanonicalFailureWireError {
    /// The encoded record contains no canonical-failure tag.
    MissingTag,
    /// The canonical-failure tag is not declared in this wire version.
    UnknownTag(u8),
    /// Bytes remain after the one-byte canonical-failure record.
    TrailingData { trailing_bytes: usize },
}

impl CanonicalFailure {
    /// Returns this failure's stable v1 wire tag.
    ///
    /// # Examples
    ///
    /// ```
    /// use moria::facade::CanonicalFailure;
    ///
    /// assert_eq!(CanonicalFailure::MissingIdentity.wire_tag(), 0);
    /// ```
    #[must_use]
    pub const fn wire_tag(self) -> u8 {
        match self {
            Self::MissingIdentity => 0,
            Self::WrongVolumeKind => 1,
            Self::StaleRevision => 2,
            Self::StaleSourceHash => 3,
            Self::InvalidBounds => 4,
            Self::InvalidCell => 5,
            Self::InvalidOrientation => 6,
            Self::InvalidFixedFormat => 7,
            Self::ArithmeticOverflow => 8,
            Self::DivisionByZero => 9,
            Self::InvalidShift => 10,
            Self::NegativeSquareRoot => 11,
            Self::Nonrepresentable => 12,
            Self::LogicalCapacity => 13,
            Self::DependencyUnavailable => 14,
            Self::ParticipantEffectInvalid => 15,
            Self::ParticipantFailed => 16,
            Self::InjectedCandidateFailure => 17,
            Self::ZeroAxis => 18,
            Self::UnrepresentableAxis => 19,
        }
    }

    /// Decodes a stable v1 canonical-failure wire tag.
    ///
    /// # Examples
    ///
    /// ```
    /// use moria::facade::CanonicalFailure;
    ///
    /// assert_eq!(
    ///     CanonicalFailure::try_from_wire_tag(18)?,
    ///     CanonicalFailure::ZeroAxis,
    /// );
    /// # Ok::<(), moria::facade::CanonicalFailureWireError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns [`CanonicalFailureWireError::UnknownTag`] when `tag` is not
    /// declared in this wire version.
    pub const fn try_from_wire_tag(tag: u8) -> Result<Self, CanonicalFailureWireError> {
        match tag {
            0 => Ok(Self::MissingIdentity),
            1 => Ok(Self::WrongVolumeKind),
            2 => Ok(Self::StaleRevision),
            3 => Ok(Self::StaleSourceHash),
            4 => Ok(Self::InvalidBounds),
            5 => Ok(Self::InvalidCell),
            6 => Ok(Self::InvalidOrientation),
            7 => Ok(Self::InvalidFixedFormat),
            8 => Ok(Self::ArithmeticOverflow),
            9 => Ok(Self::DivisionByZero),
            10 => Ok(Self::InvalidShift),
            11 => Ok(Self::NegativeSquareRoot),
            12 => Ok(Self::Nonrepresentable),
            13 => Ok(Self::LogicalCapacity),
            14 => Ok(Self::DependencyUnavailable),
            15 => Ok(Self::ParticipantEffectInvalid),
            16 => Ok(Self::ParticipantFailed),
            17 => Ok(Self::InjectedCandidateFailure),
            18 => Ok(Self::ZeroAxis),
            19 => Ok(Self::UnrepresentableAxis),
            _ => Err(CanonicalFailureWireError::UnknownTag(tag)),
        }
    }

    /// Decodes an exact one-byte canonical-failure record.
    ///
    /// # Examples
    ///
    /// ```
    /// use moria::facade::CanonicalFailure;
    ///
    /// assert_eq!(
    ///     CanonicalFailure::try_from_wire_bytes(&[19])?,
    ///     CanonicalFailure::UnrepresentableAxis,
    /// );
    /// # Ok::<(), moria::facade::CanonicalFailureWireError>(())
    /// ```
    ///
    /// # Errors
    ///
    /// Returns a typed error for a missing tag, an unknown/corrupt tag, or
    /// trailing data. It never accepts a prefix of a larger record.
    pub const fn try_from_wire_bytes(bytes: &[u8]) -> Result<Self, CanonicalFailureWireError> {
        match bytes {
            [] => Err(CanonicalFailureWireError::MissingTag),
            [tag] => Self::try_from_wire_tag(*tag),
            [_tag, trailing @ ..] => Err(CanonicalFailureWireError::TrailingData {
                trailing_bytes: trailing.len(),
            }),
        }
    }

    const fn error_code(self) -> ErrorCode {
        match self {
            Self::MissingIdentity => ErrorCode::MissingId,
            Self::WrongVolumeKind => ErrorCode::ContractMismatch,
            Self::StaleRevision => ErrorCode::StaleRevision,
            Self::StaleSourceHash => ErrorCode::StaleHash,
            Self::InvalidBounds => ErrorCode::InvalidBounds,
            Self::InvalidCell => ErrorCode::InvalidCell,
            Self::InvalidOrientation => ErrorCode::InvalidOrientation,
            Self::InvalidFixedFormat => ErrorCode::InvalidEncoding,
            Self::ArithmeticOverflow
            | Self::DivisionByZero
            | Self::InvalidShift
            | Self::NegativeSquareRoot
            | Self::Nonrepresentable
            | Self::ZeroAxis
            | Self::UnrepresentableAxis => ErrorCode::ArithmeticOverflow,
            Self::LogicalCapacity => ErrorCode::CanonicalBudget,
            Self::DependencyUnavailable => ErrorCode::DependencyNotReady,
            Self::ParticipantEffectInvalid => ErrorCode::InvalidEncoding,
            Self::ParticipantFailed => ErrorCode::ParticipantFailure,
            Self::InjectedCandidateFailure => ErrorCode::InternalInvariant,
        }
    }
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
    /// The exact unavailable half-open range in the queried volume's local cells.
    pub local: LocalCellAabb,
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
    fn matches(&self, code: AdmissionCode) -> bool {
        match (code, self) {
            (
                AdmissionCode::BeforeNextTick,
                Self::TickEligibility {
                    supplied,
                    expected_next,
                },
            ) => supplied < expected_next,
            (
                AdmissionCode::AfterNextTick,
                Self::TickEligibility {
                    supplied,
                    expected_next,
                },
            ) => supplied > expected_next,
            (AdmissionCode::InvalidBatch, Self::InvalidBatch { .. }) => true,
            (
                AdmissionCode::InterestTooLarge,
                Self::InterestCapacity {
                    required,
                    supported,
                },
            ) => required.bricks > supported.bricks,
            (
                AdmissionCode::ResultCapacityExceeded,
                Self::QueryCapacity {
                    required,
                    supported,
                },
            ) => query_capacity_exceeded(*required, *supported),
            (
                AdmissionCode::CorrectionHashCountMismatch,
                Self::CorrectionExpectedHashCount {
                    replacement_batches,
                    expected_hashes,
                },
            ) => {
                *replacement_batches != 0
                    && *expected_hashes != 0
                    && replacement_batches != expected_hashes
            }
            (
                AdmissionCode::RetiredReplayStreamCapacity,
                Self::BudgetCapacity {
                    field,
                    required,
                    supported,
                },
            ) => {
                field.group == BudgetGroup::Identity
                    && field.field_code == 2
                    && required > supported
            }
            (_, Self::None) => !matches!(
                code,
                AdmissionCode::BeforeNextTick
                    | AdmissionCode::AfterNextTick
                    | AdmissionCode::InvalidBatch
                    | AdmissionCode::InterestTooLarge
                    | AdmissionCode::ResultCapacityExceeded
                    | AdmissionCode::CorrectionHashCountMismatch
                    | AdmissionCode::RetiredReplayStreamCapacity
            ),
            _ => false,
        }
    }
}

const fn query_capacity_exceeded(required: QueryCapacity, supported: QueryCapacity) -> bool {
    required.bricks > supported.bricks
        || required.records > supported.records
        || required.result_bytes > supported.result_bytes
        || required.workgroups > supported.workgroups
        || required.volume_revisions > supported.volume_revisions
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
