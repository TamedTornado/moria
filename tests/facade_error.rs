use moria::canonical::{CanonicalHash, DeviceGeneration, ParticipantId, Tick, WorldId};
use moria::facade::{
    AdmissionCode, AdmissionContext, AdmissionError, AuthorityStatus, CanonicalFailure,
    CommittedEffect, CorrectionError, ErrorCode, FailedNoAdvance, FailureRecordError, FailureScope,
    FrontierPosition, FrontierSummary, OperationError, ProviderId, QueryUnavailable, Retryability,
    TelemetryError, TickNoAdvanceCause,
};

fn world() -> WorldId {
    WorldId::from_bytes([7; 16])
}

fn frontier(position: FrontierPosition) -> FrontierSummary {
    FrontierSummary {
        world: world(),
        position,
        root_hash: CanonicalHash::from_bytes([9; 32]),
        status: AuthorityStatus::ReplayGrade,
    }
}

fn operation(scope: FailureScope, code: ErrorCode, committed: CommittedEffect) -> OperationError {
    OperationError::new(
        code,
        scope,
        Retryability::RetryNewRequest,
        committed,
        "fixture",
    )
    .unwrap()
}

#[test]
fn admission_context_must_match_its_code() {
    let context = AdmissionContext::TickEligibility {
        supplied: Tick::from_raw(3),
        expected_next: Tick::from_raw(2),
    };

    assert!(
        AdmissionError::try_new(AdmissionCode::BeforeNextTick, Retryability::Never, context)
            .is_ok()
    );
    assert_eq!(
        AdmissionError::try_new(
            AdmissionCode::InvalidBatch,
            Retryability::Never,
            AdmissionContext::None,
        ),
        Err(FailureRecordError::AdmissionContextMismatch),
    );
}

#[test]
fn failed_no_advance_requires_one_uncommitted_matching_tick_failure() {
    let source = frontier(FrontierPosition::Confirmed(Tick::from_raw(4)));
    let participant = ParticipantId::try_from_raw(2).unwrap();
    let cause = TickNoAdvanceCause::Participant {
        participant,
        code: ErrorCode::ParticipantFailure,
    };
    let matching = operation(
        FailureScope::Tick {
            world: world(),
            tick: Tick::from_raw(5),
        },
        ErrorCode::ParticipantFailure,
        CommittedEffect::None,
    );

    assert!(FailedNoAdvance::try_new(world(), Tick::from_raw(5), source, cause, matching).is_ok());

    let committed = operation(
        FailureScope::Tick {
            world: world(),
            tick: Tick::from_raw(5),
        },
        ErrorCode::ParticipantFailure,
        CommittedEffect::Frontier(frontier(FrontierPosition::Confirmed(Tick::from_raw(5)))),
    );
    assert_eq!(
        FailedNoAdvance::try_new(
            world(),
            Tick::from_raw(5),
            source,
            TickNoAdvanceCause::Participant {
                participant,
                code: ErrorCode::ParticipantFailure,
            },
            committed,
        ),
        Err(FailureRecordError::NoAdvanceCommittedEffect),
    );

    let inconsistent = operation(
        FailureScope::Tick {
            world: world(),
            tick: Tick::from_raw(5),
        },
        ErrorCode::DeviceLost,
        CommittedEffect::None,
    );
    assert_eq!(
        FailedNoAdvance::try_new(
            world(),
            Tick::from_raw(5),
            source,
            TickNoAdvanceCause::Participant {
                participant,
                code: ErrorCode::ParticipantFailure,
            },
            inconsistent,
        ),
        Err(FailureRecordError::NoAdvanceCauseCode),
    );
}

#[test]
fn correction_error_rejects_a_committed_frontier() {
    let original = frontier(FrontierPosition::Confirmed(Tick::from_raw(4)));
    let error = operation(
        FailureScope::Operation(moria::canonical::ReceiptId::from_raw(12)),
        ErrorCode::StoreFailure,
        CommittedEffect::Frontier(frontier(FrontierPosition::Confirmed(Tick::from_raw(5)))),
    );

    assert_eq!(
        CorrectionError::try_new(original, error, None),
        Err(FailureRecordError::CorrectionCommittedEffect),
    );
}

#[test]
fn closed_failure_variants_remain_pattern_matchable() {
    let code = ErrorCode::DeviceLost;
    let cause = TickNoAdvanceCause::Device {
        generation: DeviceGeneration::from_raw(3),
        code,
    };
    let result = match cause {
        TickNoAdvanceCause::Canonical(_) => 0,
        TickNoAdvanceCause::Participant { .. } => 1,
        TickNoAdvanceCause::Provider { .. } => 2,
        TickNoAdvanceCause::Device { generation, code } => {
            assert_eq!(generation.get(), 3);
            assert_eq!(code, ErrorCode::DeviceLost);
            3
        }
        TickNoAdvanceCause::Shutdown => 4,
        TickNoAdvanceCause::Internal(_) => 5,
    };
    assert_eq!(result, 3);
}

// These exhaustive matches are compile-time guards against silently adding a
// fieldless or open-ended replacement for one of TECH-027's stable taxonomies.
#[allow(dead_code)]
fn pattern_matches_all_error_codes(code: ErrorCode) -> u8 {
    match code {
        ErrorCode::InvalidConfig
        | ErrorCode::DuplicateId
        | ErrorCode::MissingId
        | ErrorCode::WrongProviderKind
        | ErrorCode::ContractMismatch
        | ErrorCode::UnsupportedVersion
        | ErrorCode::InvalidBounds
        | ErrorCode::InvalidEncoding
        | ErrorCode::InvalidOrientation
        | ErrorCode::InvalidCell
        | ErrorCode::WrongWorld
        | ErrorCode::WorldUnknown
        | ErrorCode::BeforeNextTick
        | ErrorCode::AfterNextTick
        | ErrorCode::AlreadyPending
        | ErrorCode::WorldNotReady
        | ErrorCode::WorldClosed
        | ErrorCode::WorldFailed
        | ErrorCode::TelemetryBusy
        | ErrorCode::AlreadyShuttingDown
        | ErrorCode::DependencyNotReady
        | ErrorCode::QueueFull
        | ErrorCode::CapacityExceeded
        | ErrorCode::CanonicalBudget
        | ErrorCode::PersistenceBackpressure
        | ErrorCode::StaleRevision
        | ErrorCode::StaleHash
        | ErrorCode::ArithmeticOverflow
        | ErrorCode::SourceUnavailable
        | ErrorCode::SourceInvalid
        | ErrorCode::ProducerDropped
        | ErrorCode::StoreFailure
        | ErrorCode::ManifestNotFound
        | ErrorCode::UnsupportedAtomicCommit
        | ErrorCode::CorruptBlob
        | ErrorCode::LineageMismatch
        | ErrorCode::FrontierUnavailable
        | ErrorCode::FrontierTooOld
        | ErrorCode::ResultCapacityExceeded
        | ErrorCode::ObservationGap
        | ErrorCode::ParticipantFailure
        | ErrorCode::ParticipantDivergence
        | ErrorCode::ReplayDivergence
        | ErrorCode::BackendUnavailable
        | ErrorCode::DeterminismViolation
        | ErrorCode::DeviceLost
        | ErrorCode::MappingFailure
        | ErrorCode::DecodeFailure
        | ErrorCode::Cancelled
        | ErrorCode::Shutdown
        | ErrorCode::InternalInvariant => 0,
    }
}

#[allow(dead_code)]
fn pattern_matches_all_admission_codes(code: AdmissionCode) -> u8 {
    match code {
        AdmissionCode::WrongWorld
        | AdmissionCode::WrongState
        | AdmissionCode::DuplicateReplayStream
        | AdmissionCode::RetiredReplayStreamCapacity
        | AdmissionCode::BeforeNextTick
        | AdmissionCode::AfterNextTick
        | AdmissionCode::AlreadyPending
        | AdmissionCode::WorldNotReady
        | AdmissionCode::DependencyNotReady
        | AdmissionCode::Full
        | AdmissionCode::Closed
        | AdmissionCode::InvalidRequest
        | AdmissionCode::InvalidBatch
        | AdmissionCode::InterestTooLarge
        | AdmissionCode::ResultCapacityExceeded
        | AdmissionCode::CorrectionHashCountMismatch
        | AdmissionCode::StaleGeneration
        | AdmissionCode::PersistenceBackpressure => 0,
    }
}

#[allow(dead_code)]
fn pattern_matches_all_canonical_failures(failure: CanonicalFailure) -> u8 {
    match failure {
        CanonicalFailure::MissingIdentity
        | CanonicalFailure::WrongVolumeKind
        | CanonicalFailure::StaleRevision
        | CanonicalFailure::StaleSourceHash
        | CanonicalFailure::InvalidBounds
        | CanonicalFailure::InvalidCell
        | CanonicalFailure::InvalidOrientation
        | CanonicalFailure::InvalidFixedFormat
        | CanonicalFailure::ArithmeticOverflow
        | CanonicalFailure::DivisionByZero
        | CanonicalFailure::InvalidShift
        | CanonicalFailure::NegativeSquareRoot
        | CanonicalFailure::Nonrepresentable
        | CanonicalFailure::LogicalCapacity
        | CanonicalFailure::DependencyUnavailable
        | CanonicalFailure::ParticipantEffectInvalid
        | CanonicalFailure::ParticipantFailed
        | CanonicalFailure::InjectedCandidateFailure
        | CanonicalFailure::ZeroAxis
        | CanonicalFailure::UnrepresentableAxis => 0,
    }
}

#[allow(dead_code)]
fn pattern_matches_all_outer_failures(error: TelemetryError, query: QueryUnavailable) -> u8 {
    let telemetry = match error {
        TelemetryError::WorldUnknown { .. } => 0,
        TelemetryError::WorldClosed { .. } => 1,
        TelemetryError::TelemetryBusy { .. } => 2,
    };
    let unavailable = match query {
        QueryUnavailable::Availability { .. } => 0,
        QueryUnavailable::ResultCapacityExceeded { .. } => 1,
    };
    telemetry + unavailable
}

#[allow(dead_code)]
fn pattern_matches_all_scopes_and_providers(scope: FailureScope, provider: ProviderId) -> u8 {
    let scope = match scope {
        FailureScope::Configuration => 0,
        FailureScope::World(_) => 1,
        FailureScope::Tick { .. } => 2,
        FailureScope::Volume { .. } => 3,
        FailureScope::Operation(_) => 4,
        FailureScope::Provider(_) => 5,
    };
    let provider = match provider {
        ProviderId::InputSource(_) => 0,
        ProviderId::BaseSource(_) => 1,
        ProviderId::BaseAuthority(_) => 2,
        ProviderId::ContentBlobStore(_) => 3,
        ProviderId::CheckpointStore(_) => 4,
        ProviderId::ReplaySink(_) => 5,
        ProviderId::Participant(_) => 6,
    };
    scope + provider
}
