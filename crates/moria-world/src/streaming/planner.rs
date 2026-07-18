#![allow(
    dead_code,
    reason = "streaming execution consumes the planner in the later composition slice"
)]

#[cfg(test)]
mod tests {
    use crate::{ActiveBand, BrickCoord, StreamingConfig};

    use super::{PlanRequest, StreamPlanner, StreamPriority};

    #[test]
    fn plans_are_priority_sorted_stable_and_do_not_reenqueue_unchanged_work() {
        let near = BrickCoord::new(125, 32, 125).unwrap();
        let far = BrickCoord::new(126, 32, 125).unwrap();
        let config = StreamingConfig::default();
        let mut planner = StreamPlanner::new(&config);

        let first = planner.plan([
            PlanRequest::new(near, Some(ActiveBand::Near), StreamPriority::Camera, 20),
            PlanRequest::new(far, Some(ActiveBand::Far), StreamPriority::Prefetch, 200),
            PlanRequest::new(
                near,
                Some(ActiveBand::Near),
                StreamPriority::CommittedEdit,
                20,
            ),
        ]);

        assert_eq!(first.len(), 2);
        assert_eq!(first[0].brick, near);
        assert_eq!(first[0].priority, StreamPriority::CommittedEdit);
        assert_eq!(first[1].brick, far);
        assert!(planner
            .plan([
                PlanRequest::new(near, Some(ActiveBand::Near), StreamPriority::Camera, 20),
                PlanRequest::new(far, Some(ActiveBand::Far), StreamPriority::Prefetch, 200),
            ])
            .is_empty());
    }

    #[test]
    fn planner_holds_band_edges_until_hysteresis_is_crossed() {
        let brick = BrickCoord::new(125, 32, 125).unwrap();
        let config = StreamingConfig::default();
        let mut planner = StreamPlanner::new(&config);

        planner.plan([PlanRequest::new(
            brick,
            Some(ActiveBand::Near),
            StreamPriority::NearVisual,
            60,
        )]);
        assert!(planner
            .plan([PlanRequest::new(
                brick,
                Some(ActiveBand::Middle),
                StreamPriority::NearVisual,
                70,
            )])
            .is_empty());
        let changed = planner.plan([PlanRequest::new(
            brick,
            Some(ActiveBand::Middle),
            StreamPriority::NearVisual,
            76,
        )]);
        assert_eq!(changed[0].band, Some(ActiveBand::Middle));
    }

    #[test]
    fn planner_uses_configured_band_edges_for_hysteresis() {
        let brick = BrickCoord::new(125, 32, 125).unwrap();
        let mut config = StreamingConfig::default();
        config.bands[0].end_m = 96;
        config.bands[1].start_m = 96;
        let mut planner = StreamPlanner::new(&config);

        planner.plan([PlanRequest::new(
            brick,
            Some(ActiveBand::Near),
            StreamPriority::NearVisual,
            90,
        )]);
        assert!(planner
            .plan([PlanRequest::new(
                brick,
                Some(ActiveBand::Middle),
                StreamPriority::NearVisual,
                107,
            )])
            .is_empty());

        let changed = planner.plan([PlanRequest::new(
            brick,
            Some(ActiveBand::Middle),
            StreamPriority::NearVisual,
            108,
        )]);
        assert_eq!(changed[0].band, Some(ActiveBand::Middle));
    }

    #[test]
    fn planner_covers_each_visual_band_and_traversal_collision_work() {
        let config = StreamingConfig::default();
        let mut planner = StreamPlanner::new(&config);
        let requests = [
            (ActiveBand::Near, StreamPriority::Collision),
            (ActiveBand::Near, StreamPriority::Traversal),
            (ActiveBand::Near, StreamPriority::Camera),
            (ActiveBand::Middle, StreamPriority::NearVisual),
            (ActiveBand::Far, StreamPriority::FarVisual),
            (ActiveBand::Horizon, StreamPriority::Prefetch),
        ]
        .into_iter()
        .enumerate()
        .map(|(index, (band, priority))| {
            PlanRequest::new(
                BrickCoord::new(125 + index as i16, 32, 125).unwrap(),
                Some(band),
                priority,
                400,
            )
        });

        let plans = planner.plan(requests);
        assert_eq!(plans.len(), 6);
        assert!(plans
            .iter()
            .any(|plan| plan.priority == StreamPriority::Collision));
        assert!(plans
            .iter()
            .any(|plan| plan.priority == StreamPriority::Traversal));
        assert!(plans.iter().any(|plan| plan.band == Some(ActiveBand::Near)));
        assert!(plans
            .iter()
            .any(|plan| plan.band == Some(ActiveBand::Middle)));
        assert!(plans.iter().any(|plan| plan.band == Some(ActiveBand::Far)));
        assert!(plans
            .iter()
            .any(|plan| plan.band == Some(ActiveBand::Horizon)));
    }
}
// Deterministic desired-band planning independent from task execution.

use std::collections::BTreeMap;

use crate::{ActiveBand, BrickCoord, StreamingConfig};

/// The reason work is ordered ahead of other streaming work.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(crate) enum StreamPriority {
    CommittedEdit,
    Collision,
    Traversal,
    Camera,
    Inspection,
    NearVisual,
    FarVisual,
    Prefetch,
}

/// One desired state supplied by focus/collision planning.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PlanRequest {
    pub(crate) brick: BrickCoord,
    pub(crate) band: Option<ActiveBand>,
    pub(crate) priority: StreamPriority,
    pub(crate) distance_m: u16,
}

impl PlanRequest {
    pub(crate) const fn new(
        brick: BrickCoord,
        band: Option<ActiveBand>,
        priority: StreamPriority,
        distance_m: u16,
    ) -> Self {
        Self {
            brick,
            band,
            priority,
            distance_m,
        }
    }
}

/// A changed desired state that must be sent to the lifecycle/task queue.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct PlannedBrick {
    pub(crate) brick: BrickCoord,
    pub(crate) band: Option<ActiveBand>,
    pub(crate) priority: StreamPriority,
    distance_m: u16,
}

/// Keeps the previously accepted band so work at a distance edge does not churn.
#[derive(Debug)]
pub(crate) struct StreamPlanner {
    hysteresis_m: u16,
    bands: [BandRange; 4],
    desired: BTreeMap<BrickCoord, ActiveBand>,
}

impl StreamPlanner {
    pub(crate) fn new(config: &StreamingConfig) -> Self {
        Self {
            hysteresis_m: u16::from(config.hysteresis_m),
            bands: std::array::from_fn(|index| BandRange {
                start_m: config.bands[index].start_m,
                end_m: config.bands[index].end_m,
            }),
            desired: BTreeMap::new(),
        }
    }

    /// Reconciles a complete desired set and returns only changed work.
    ///
    /// BTree ordering makes duplicate coalescing and equal-priority output stable.
    pub(crate) fn plan(
        &mut self,
        requests: impl IntoIterator<Item = PlanRequest>,
    ) -> Vec<PlannedBrick> {
        let mut requested = BTreeMap::<BrickCoord, PlanRequest>::new();
        for request in requests {
            requested
                .entry(request.brick)
                .and_modify(|current| {
                    if (request.priority, request.distance_m, request.band)
                        < (current.priority, current.distance_m, current.band)
                    {
                        *current = request;
                    }
                })
                .or_insert(request);
        }

        let mut changes = Vec::new();
        for request in requested.values().copied() {
            let current = self.desired.get(&request.brick).copied();
            let accepted = self.apply_hysteresis(current, request.band, request.distance_m);
            if accepted != current {
                match accepted {
                    Some(band) => {
                        self.desired.insert(request.brick, band);
                    }
                    None => {
                        self.desired.remove(&request.brick);
                    }
                }
                changes.push(PlannedBrick {
                    brick: request.brick,
                    band: accepted,
                    priority: request.priority,
                    distance_m: request.distance_m,
                });
            }
        }

        let absent = self
            .desired
            .keys()
            .copied()
            .filter(|brick| !requested.contains_key(brick))
            .collect::<Vec<_>>();
        for brick in absent {
            self.desired.remove(&brick);
            changes.push(PlannedBrick {
                brick,
                band: None,
                priority: StreamPriority::Prefetch,
                distance_m: u16::MAX,
            });
        }

        changes.sort_unstable_by_key(|change| (change.priority, change.distance_m, change.brick));
        changes
    }

    fn apply_hysteresis(
        &self,
        current: Option<ActiveBand>,
        requested: Option<ActiveBand>,
        distance_m: u16,
    ) -> Option<ActiveBand> {
        let Some(current) = current else {
            return requested;
        };
        let Some(requested) = requested else {
            return (current == ActiveBand::Horizon
                && distance_m <= self.band(ActiveBand::Horizon).end_m + self.hysteresis_m)
                .then_some(current);
        };
        if current == requested {
            return Some(current);
        }
        if requested > current {
            (distance_m >= self.band(requested).start_m + self.hysteresis_m)
                .then_some(requested)
                .or(Some(current))
        } else {
            (distance_m <= self.band(current).start_m.saturating_sub(self.hysteresis_m))
                .then_some(requested)
                .or(Some(current))
        }
    }

    const fn band(&self, band: ActiveBand) -> BandRange {
        self.bands[match band {
            ActiveBand::Near => 0,
            ActiveBand::Middle => 1,
            ActiveBand::Far => 2,
            ActiveBand::Horizon => 3,
        }]
    }
}

#[derive(Clone, Copy, Debug)]
struct BandRange {
    start_m: u16,
    end_m: u16,
}
