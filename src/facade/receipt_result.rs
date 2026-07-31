//! Concrete bounded result records returned by public receipt families.

use crate::canonical::Tick;

use super::{FrontierSummary, ReplayStreamKey};

/// The replay-stream position made durable by genesis.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ReplayStreamPosition {
    pub stream: ReplayStreamKey,
    pub sequence: u64,
}

/// The frontier published by a successful genesis operation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct GenesisReady {
    pub frontier: FrontierSummary,
    pub next_tick: Tick,
    pub replay: ReplayStreamPosition,
}

macro_rules! opaque_receipt_result {
    ($($name:ident),+ $(,)?) => {$(
        #[doc = concat!("The bounded ready record for a successful `", stringify!($name), "` operation.")]
        #[derive(Debug, Eq, PartialEq)]
        pub(crate) struct $name {
            _private: (),
        }

        impl $name {
            #[allow(dead_code)]
            pub(crate) const fn new() -> Self {
                Self { _private: () }
            }
        }
    )+};
}

opaque_receipt_result!(
    TickConfirmed,
    InterestApplied,
    QueryResult,
    ObservationResnapshot,
    CheckpointCommitted,
    CorrectionCommitted,
    RestoreReady,
    ReplayCompleted,
    Recovered,
    ShutdownReport,
);
