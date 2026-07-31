use moria::canonical::{
    BlobDigest, CanonicalHash, DeviceGeneration, LocalCellAabb, LocalCellPoint, ParticipantId,
    Tick, VolumeId, WorldId,
};
use moria::facade::{
    AdmissionCode, AdmissionContext, AdmissionError, AuthorityStatus, AvailabilityCode, BatchError,
    BudgetGroup, CanonicalFailure, CanonicalFailureWireError, CheckpointError, CommittedEffect,
    ConfigErrorCode, ConfigField, CorrectionError, ErrorCode, FailedNoAdvance, FailureRecordError,
    FailureScope, FrontierPosition, FrontierSummary, GenesisError, InterestCapacity, InterestError,
    ObservationSnapshotError, OperationError, ParticipantError, ProviderId, QueryCapacity,
    QueryUnavailable, RecoveryError, ReplayAppendRange, ReplayExportFailure, ReplaySinkId,
    ReplaySinkRequest, ReplayStreamKey, ResourceBudgetField, ResourceBudgetFieldError,
    RestoreError, Retryability, ShutdownError, TelemetryError, TickNoAdvanceCause,
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
        supplied: Tick::from_raw(1),
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
fn admission_context_requires_consistent_boundary_facts() {
    let invalid = |code, context| {
        assert_eq!(
            AdmissionError::try_new(code, Retryability::Never, context),
            Err(FailureRecordError::AdmissionContextMismatch),
        );
    };
    let query_capacity = |bricks| QueryCapacity {
        bricks,
        records: 0,
        result_bytes: 0,
        workgroups: 0,
        volume_revisions: 0,
    };

    invalid(
        AdmissionCode::BeforeNextTick,
        AdmissionContext::TickEligibility {
            supplied: Tick::from_raw(2),
            expected_next: Tick::from_raw(2),
        },
    );
    invalid(
        AdmissionCode::BeforeNextTick,
        AdmissionContext::TickEligibility {
            supplied: Tick::from_raw(3),
            expected_next: Tick::from_raw(2),
        },
    );
    invalid(
        AdmissionCode::AfterNextTick,
        AdmissionContext::TickEligibility {
            supplied: Tick::from_raw(2),
            expected_next: Tick::from_raw(2),
        },
    );
    invalid(
        AdmissionCode::AfterNextTick,
        AdmissionContext::TickEligibility {
            supplied: Tick::from_raw(1),
            expected_next: Tick::from_raw(2),
        },
    );
    invalid(
        AdmissionCode::CorrectionHashCountMismatch,
        AdmissionContext::CorrectionExpectedHashCount {
            replacement_batches: 2,
            expected_hashes: 2,
        },
    );
    invalid(
        AdmissionCode::InterestTooLarge,
        AdmissionContext::InterestCapacity {
            required: InterestCapacity { bricks: 2 },
            supported: InterestCapacity { bricks: 2 },
        },
    );
    invalid(
        AdmissionCode::ResultCapacityExceeded,
        AdmissionContext::QueryCapacity {
            required: query_capacity(2),
            supported: query_capacity(2),
        },
    );
    invalid(
        AdmissionCode::RetiredReplayStreamCapacity,
        AdmissionContext::BudgetCapacity {
            field: ResourceBudgetField::try_new(BudgetGroup::Identity, 2).unwrap(),
            required: 2,
            supported: 2,
        },
    );
    invalid(
        AdmissionCode::RetiredReplayStreamCapacity,
        AdmissionContext::BudgetCapacity {
            field: ResourceBudgetField::try_new(BudgetGroup::Identity, 1).unwrap(),
            required: 2,
            supported: 1,
        },
    );

    assert!(
        AdmissionError::try_new(
            AdmissionCode::AfterNextTick,
            Retryability::Never,
            AdmissionContext::TickEligibility {
                supplied: Tick::from_raw(3),
                expected_next: Tick::from_raw(2),
            },
        )
        .is_ok()
    );
    assert!(
        AdmissionError::try_new(
            AdmissionCode::RetiredReplayStreamCapacity,
            Retryability::Never,
            AdmissionContext::BudgetCapacity {
                field: ResourceBudgetField::try_new(BudgetGroup::Identity, 2).unwrap(),
                required: 2,
                supported: 1,
            },
        )
        .is_ok()
    );
    assert!(
        AdmissionError::try_new(
            AdmissionCode::ResultCapacityExceeded,
            Retryability::Never,
            AdmissionContext::QueryCapacity {
                required: QueryCapacity {
                    records: 1,
                    ..query_capacity(0)
                },
                supported: query_capacity(0),
            },
        )
        .is_ok()
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

    let scope = FailureScope::Tick {
        world: world(),
        tick: Tick::from_raw(5),
    };
    for (failure, code) in [
        (CanonicalFailure::MissingIdentity, ErrorCode::MissingId),
        (
            CanonicalFailure::WrongVolumeKind,
            ErrorCode::ContractMismatch,
        ),
        (CanonicalFailure::StaleRevision, ErrorCode::StaleRevision),
        (CanonicalFailure::StaleSourceHash, ErrorCode::StaleHash),
        (CanonicalFailure::InvalidBounds, ErrorCode::InvalidBounds),
        (CanonicalFailure::InvalidCell, ErrorCode::InvalidCell),
        (
            CanonicalFailure::InvalidOrientation,
            ErrorCode::InvalidOrientation,
        ),
        (
            CanonicalFailure::InvalidFixedFormat,
            ErrorCode::InvalidEncoding,
        ),
        (
            CanonicalFailure::ArithmeticOverflow,
            ErrorCode::ArithmeticOverflow,
        ),
        (
            CanonicalFailure::DivisionByZero,
            ErrorCode::ArithmeticOverflow,
        ),
        (
            CanonicalFailure::InvalidShift,
            ErrorCode::ArithmeticOverflow,
        ),
        (
            CanonicalFailure::NegativeSquareRoot,
            ErrorCode::ArithmeticOverflow,
        ),
        (
            CanonicalFailure::Nonrepresentable,
            ErrorCode::ArithmeticOverflow,
        ),
        (
            CanonicalFailure::LogicalCapacity,
            ErrorCode::CanonicalBudget,
        ),
        (
            CanonicalFailure::DependencyUnavailable,
            ErrorCode::DependencyNotReady,
        ),
        (
            CanonicalFailure::ParticipantEffectInvalid,
            ErrorCode::InvalidEncoding,
        ),
        (
            CanonicalFailure::ParticipantFailed,
            ErrorCode::ParticipantFailure,
        ),
        (
            CanonicalFailure::InjectedCandidateFailure,
            ErrorCode::InternalInvariant,
        ),
        (CanonicalFailure::ZeroAxis, ErrorCode::ArithmeticOverflow),
        (
            CanonicalFailure::UnrepresentableAxis,
            ErrorCode::ArithmeticOverflow,
        ),
    ] {
        assert!(
            FailedNoAdvance::try_new(
                world(),
                Tick::from_raw(5),
                source,
                TickNoAdvanceCause::Canonical(failure),
                operation(scope, code, CommittedEffect::None),
            )
            .is_ok()
        );
    }
}

#[test]
fn canonical_no_advance_cause_requires_its_declared_error_code() {
    let source = frontier(FrontierPosition::Confirmed(Tick::from_raw(4)));
    let scope = FailureScope::Tick {
        world: world(),
        tick: Tick::from_raw(5),
    };

    assert!(
        FailedNoAdvance::try_new(
            world(),
            Tick::from_raw(5),
            source,
            TickNoAdvanceCause::Canonical(CanonicalFailure::StaleRevision),
            operation(scope, ErrorCode::StaleRevision, CommittedEffect::None),
        )
        .is_ok()
    );

    assert_eq!(
        FailedNoAdvance::try_new(
            world(),
            Tick::from_raw(5),
            source,
            TickNoAdvanceCause::Canonical(CanonicalFailure::StaleRevision),
            operation(scope, ErrorCode::DeviceLost, CommittedEffect::None),
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
fn correction_error_preserves_only_legal_correction_branch_export_failures() {
    let original = frontier(FrontierPosition::Confirmed(Tick::from_raw(4)));
    let sink = ReplaySinkId::try_from_raw(1).unwrap();
    let stream = ReplayStreamKey::try_from_bytes([3; 32]).unwrap();
    let error = || {
        OperationError::new(
            ErrorCode::StoreFailure,
            FailureScope::Provider(ProviderId::ReplaySink(sink)),
            Retryability::Never,
            CommittedEffect::None,
            "fixture",
        )
        .unwrap()
    };
    let export = |range, failure| ReplayExportFailure {
        sink,
        request: ReplaySinkRequest {
            stream,
            sequence: 4,
            range,
            bytes: 32,
            digest: BlobDigest::from_bytes([4; 32]),
        },
        failure,
    };

    for failure in [
        ErrorCode::StoreFailure,
        ErrorCode::ProducerDropped,
        ErrorCode::ContractMismatch,
    ] {
        assert_eq!(
            CorrectionError::try_new(
                original,
                error(),
                Some(export(
                    ReplayAppendRange::CorrectionBranch {
                        target_tick: Tick::from_raw(2),
                        superseded_through: Tick::from_raw(4),
                        corrected_through: Tick::from_raw(4),
                        record_count: 3,
                    },
                    failure,
                )),
            ),
            Ok(CorrectionError {
                original_frontier: original,
                error: error(),
                replay_export_failure: Some(export(
                    ReplayAppendRange::CorrectionBranch {
                        target_tick: Tick::from_raw(2),
                        superseded_through: Tick::from_raw(4),
                        corrected_through: Tick::from_raw(4),
                        record_count: 3,
                    },
                    failure,
                )),
            }),
        );
    }
    assert_eq!(
        CorrectionError::try_new(
            original,
            error(),
            Some(export(
                ReplayAppendRange::CorrectionBranch {
                    target_tick: Tick::from_raw(2),
                    superseded_through: Tick::from_raw(4),
                    corrected_through: Tick::from_raw(4),
                    record_count: 3,
                },
                ErrorCode::DeviceLost
            )),
        ),
        Err(FailureRecordError::CorrectionExportFailure),
    );
    assert_eq!(
        CorrectionError::try_new(
            original,
            error(),
            Some(export(
                ReplayAppendRange::Header {
                    starting: FrontierPosition::Genesis,
                    next_tick: Tick::from_raw(0),
                },
                ErrorCode::ProducerDropped
            )),
        ),
        Err(FailureRecordError::CorrectionExportFailure),
    );
    assert_eq!(
        CorrectionError::try_new(original, error(), None),
        Err(FailureRecordError::CorrectionExportFailure),
    );
}

#[test]
fn canonical_failure_wire_tags_are_stable_and_exactly_decoded() {
    let declared = [
        CanonicalFailure::MissingIdentity,
        CanonicalFailure::WrongVolumeKind,
        CanonicalFailure::StaleRevision,
        CanonicalFailure::StaleSourceHash,
        CanonicalFailure::InvalidBounds,
        CanonicalFailure::InvalidCell,
        CanonicalFailure::InvalidOrientation,
        CanonicalFailure::InvalidFixedFormat,
        CanonicalFailure::ArithmeticOverflow,
        CanonicalFailure::DivisionByZero,
        CanonicalFailure::InvalidShift,
        CanonicalFailure::NegativeSquareRoot,
        CanonicalFailure::Nonrepresentable,
        CanonicalFailure::LogicalCapacity,
        CanonicalFailure::DependencyUnavailable,
        CanonicalFailure::ParticipantEffectInvalid,
        CanonicalFailure::ParticipantFailed,
        CanonicalFailure::InjectedCandidateFailure,
        CanonicalFailure::ZeroAxis,
        CanonicalFailure::UnrepresentableAxis,
    ];
    for (tag, failure) in declared.into_iter().enumerate() {
        assert_eq!(failure.wire_tag(), tag as u8);
        assert_eq!(CanonicalFailure::try_from_wire_tag(tag as u8), Ok(failure));
        assert_eq!(
            CanonicalFailure::try_from_wire_bytes(&[tag as u8]),
            Ok(failure)
        );
    }
    assert_eq!(
        CanonicalFailure::try_from_wire_tag(20),
        Err(CanonicalFailureWireError::UnknownTag(20)),
    );
    assert_eq!(
        CanonicalFailure::try_from_wire_bytes(&[20]),
        Err(CanonicalFailureWireError::UnknownTag(20)),
    );
    assert_eq!(
        CanonicalFailure::try_from_wire_bytes(&[]),
        Err(CanonicalFailureWireError::MissingTag),
    );
    assert_eq!(
        CanonicalFailure::try_from_wire_bytes(&[0, 1]),
        Err(CanonicalFailureWireError::TrailingData { trailing_bytes: 1 }),
    );
}

#[test]
fn query_missing_ranges_retain_the_exact_local_range() {
    let local = LocalCellAabb::try_new(
        LocalCellPoint([-2, 3, 4]),
        LocalCellPoint([1, 5, 6]),
        LocalCellPoint([0, 0, 0]),
    )
    .unwrap();
    let unavailable = QueryUnavailable::Availability {
        error: operation(
            FailureScope::World(world()),
            ErrorCode::SourceUnavailable,
            CommittedEffect::None,
        ),
        missing: moria::facade::BoundedVec::try_from_vec(
            vec![moria::facade::MissingRange {
                volume: VolumeId::try_from_raw(1).unwrap(),
                local,
                reason: AvailabilityCode::Cold,
            }],
            1,
        )
        .unwrap(),
    };
    let QueryUnavailable::Availability { missing, .. } = unavailable else {
        panic!("expected availability");
    };
    assert_eq!(missing.as_slice()[0].local, local);
}

#[test]
fn operation_diagnostic_is_rejected_before_copying_an_over_limit_input() {
    assert_eq!(
        OperationError::new(
            ErrorCode::InvalidConfig,
            FailureScope::Configuration,
            Retryability::Never,
            CommittedEffect::None,
            &"x".repeat(161),
        ),
        Err(FailureRecordError::Diagnostic(
            moria::facade::BoundedOwnerError::LengthExceedsCapacity,
        )),
    );
}

#[test]
fn resource_budget_field_rejects_unknown_group_tags_and_ordinals() {
    let declared = [
        (BudgetGroup::Identity, 19),
        (BudgetGroup::Canonical, 8),
        (BudgetGroup::Content, 9),
        (BudgetGroup::Query, 7),
        (BudgetGroup::Observation, 10),
        (BudgetGroup::Presentation, 8),
        (BudgetGroup::Checkpoint, 10),
        (BudgetGroup::Rollback, 18),
        (BudgetGroup::Participant, 11),
        (BudgetGroup::Runtime, 4),
    ];
    for (tag, (group, final_ordinal)) in declared.into_iter().enumerate() {
        assert_eq!(
            ResourceBudgetField::try_from_wire_parts(tag as u8, final_ordinal),
            Ok(ResourceBudgetField {
                group,
                field_code: final_ordinal,
            }),
        );
        assert_eq!(
            ResourceBudgetField::try_new(group, final_ordinal + 1),
            Err(ResourceBudgetFieldError::UnknownFieldOrdinal {
                group,
                field_code: final_ordinal + 1,
            }),
        );
    }
    assert_eq!(
        ResourceBudgetField::try_from_wire_parts(10, 1),
        Err(ResourceBudgetFieldError::UnknownGroupTag(10)),
    );
}

#[test]
fn operation_receipt_error_aliases_are_public_operation_errors() {
    let _: GenesisError = operation(
        FailureScope::Configuration,
        ErrorCode::InvalidConfig,
        CommittedEffect::None,
    );
    let _: InterestError = operation(
        FailureScope::Configuration,
        ErrorCode::InvalidConfig,
        CommittedEffect::None,
    );
    let _: ObservationSnapshotError = operation(
        FailureScope::Configuration,
        ErrorCode::InvalidConfig,
        CommittedEffect::None,
    );
    let _: CheckpointError = operation(
        FailureScope::Configuration,
        ErrorCode::InvalidConfig,
        CommittedEffect::None,
    );
    let _: RestoreError = operation(
        FailureScope::Configuration,
        ErrorCode::InvalidConfig,
        CommittedEffect::None,
    );
    let _: ParticipantError = operation(
        FailureScope::Configuration,
        ErrorCode::InvalidConfig,
        CommittedEffect::None,
    );
    let _: RecoveryError = operation(
        FailureScope::Configuration,
        ErrorCode::InvalidConfig,
        CommittedEffect::None,
    );
    let _: ShutdownError = operation(
        FailureScope::Configuration,
        ErrorCode::InvalidConfig,
        CommittedEffect::None,
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

#[allow(dead_code)]
fn pattern_matches_all_retryability(value: Retryability) -> u8 {
    match value {
        Retryability::RetryNewRequest => 0,
        Retryability::RetryAfterDependency => 1,
        Retryability::RetryAfterRecovery => 2,
        Retryability::Never => 3,
    }
}

#[allow(dead_code)]
fn pattern_matches_all_committed_effects(value: CommittedEffect) -> u8 {
    match value {
        CommittedEffect::None => 0,
        CommittedEffect::Frontier(_) => 1,
    }
}

#[allow(dead_code)]
fn pattern_matches_all_admission_contexts(value: AdmissionContext) -> u8 {
    match value {
        AdmissionContext::None => 0,
        AdmissionContext::TickEligibility { .. } => 1,
        AdmissionContext::InvalidBatch { .. } => 2,
        AdmissionContext::InterestCapacity { .. } => 3,
        AdmissionContext::QueryCapacity { .. } => 4,
        AdmissionContext::CorrectionExpectedHashCount { .. } => 5,
        AdmissionContext::BudgetCapacity { .. } => 6,
    }
}

#[allow(dead_code)]
fn pattern_matches_all_budget_and_config_variants(group: BudgetGroup, field: ConfigField) -> u8 {
    let group = match group {
        BudgetGroup::Identity => 0,
        BudgetGroup::Canonical => 1,
        BudgetGroup::Content => 2,
        BudgetGroup::Query => 3,
        BudgetGroup::Observation => 4,
        BudgetGroup::Presentation => 5,
        BudgetGroup::Checkpoint => 6,
        BudgetGroup::Rollback => 7,
        BudgetGroup::Participant => 8,
        BudgetGroup::Runtime => 9,
    };
    let field = match field {
        ConfigField::Canonical => 0,
        ConfigField::Budgets(_) => 1,
        ConfigField::Rollback => 2,
        ConfigField::Persistence => 3,
        ConfigField::Presentation => 4,
        ConfigField::Execution => 5,
        ConfigField::Material => 6,
        ConfigField::InputSource => 7,
        ConfigField::BaseSource => 8,
        ConfigField::BaseAuthority => 9,
        ConfigField::ContentBlobStore => 10,
        ConfigField::CheckpointStore => 11,
        ConfigField::ReplaySink => 12,
        ConfigField::Volume => 13,
        ConfigField::Participant => 14,
    };
    group + field
}

#[allow(dead_code)]
fn pattern_matches_all_config_error_codes(value: ConfigErrorCode) -> u8 {
    match value {
        ConfigErrorCode::DuplicateId => 0,
        ConfigErrorCode::RetiredReplayStreamCapacity => 1,
        ConfigErrorCode::MissingReference => 2,
        ConfigErrorCode::WrongProviderKind => 3,
        ConfigErrorCode::ContractMismatch => 4,
        ConfigErrorCode::InvalidValue => 5,
        ConfigErrorCode::CrossLimitViolation => 6,
        ConfigErrorCode::UnsupportedCapability => 7,
        ConfigErrorCode::ArithmeticOverflow => 8,
    }
}

#[allow(dead_code)]
fn pattern_matches_all_replay_and_availability_variants(
    range: ReplayAppendRange,
    availability: AvailabilityCode,
) -> u8 {
    let range = match range {
        ReplayAppendRange::Header { .. } => 0,
        ReplayAppendRange::TickRecords { .. } => 1,
        ReplayAppendRange::CorrectionBranch { .. } => 2,
    };
    let availability = match availability {
        AvailabilityCode::Cold => 0,
        AvailabilityCode::Materializing => 1,
        AvailabilityCode::Failed => 2,
        AvailabilityCode::FrontierTooOld => 3,
        AvailabilityCode::DeviceLost => 4,
        AvailabilityCode::CapacityExceeded => 5,
    };
    range + availability
}

#[allow(dead_code)]
fn pattern_matches_all_batch_errors(value: BatchError) -> u8 {
    match value {
        BatchError::Empty => 0,
        BatchError::CountMismatch => 1,
        BatchError::DuplicateCanonicalKey => 2,
        BatchError::EncodingFailure => 3,
        BatchError::ReservationMismatch => 4,
    }
}

#[allow(dead_code)]
fn pattern_matches_all_frontier_variants(
    position: FrontierPosition,
    status: AuthorityStatus,
) -> u8 {
    let position = match position {
        FrontierPosition::Genesis => 0,
        FrontierPosition::Confirmed(_) => 1,
    };
    let status = match status {
        AuthorityStatus::ReplayGrade => 0,
        AuthorityStatus::DiagnosticCandidate => 1,
    };
    position + status
}

#[allow(dead_code)]
fn pattern_matches_all_failure_record_errors(value: FailureRecordError) -> u8 {
    match value {
        FailureRecordError::Diagnostic(_) => 0,
        FailureRecordError::AdmissionContextMismatch => 1,
        FailureRecordError::NoAdvanceSourceFrontier => 2,
        FailureRecordError::NoAdvanceScope => 3,
        FailureRecordError::NoAdvanceCauseCode => 4,
        FailureRecordError::NoAdvanceCommittedEffect => 5,
        FailureRecordError::CorrectionCommittedEffect => 6,
        FailureRecordError::CorrectionExportFailure => 7,
    }
}

#[allow(dead_code)]
fn pattern_matches_all_budget_field_errors(value: ResourceBudgetFieldError) -> u8 {
    match value {
        ResourceBudgetFieldError::UnknownGroupTag(_) => 0,
        ResourceBudgetFieldError::UnknownFieldOrdinal { .. } => 1,
    }
}
