//! Structural supervision scenario evidence.
//!
//! These fixtures exercise only the publish-disabled observation boundary. They
//! do not execute Actors, restart children, consume mailboxes, or assert
//! recovery outcomes.

use std::collections::BTreeSet;

use ling_concurrency::{
    ActorId, SupervisorObservation, SupervisorObservationId, SupervisorObservationKind,
    SupervisorObservationModel, SupervisorObservationSpec,
};

#[derive(Clone, Copy)]
struct ObservationFixture {
    id: u32,
    actor: Option<u32>,
    kind: SupervisorObservationKind,
}

const SCENARIOS: &[(&str, &[ObservationFixture])] = &[
    (
        "single-child-fault",
        &[ObservationFixture {
            id: 1,
            actor: Some(11),
            kind: SupervisorObservationKind::FaultChannel,
        }],
    ),
    (
        "multiple-child-faults",
        &[
            ObservationFixture {
                id: 1,
                actor: Some(11),
                kind: SupervisorObservationKind::FaultChannel,
            },
            ObservationFixture {
                id: 2,
                actor: Some(12),
                kind: SupervisorObservationKind::FaultChannel,
            },
            ObservationFixture {
                id: 3,
                actor: None,
                kind: SupervisorObservationKind::ChildSpec,
            },
        ],
    ),
    (
        "fault-during-restart",
        &[
            ObservationFixture {
                id: 1,
                actor: Some(11),
                kind: SupervisorObservationKind::Restart,
            },
            ObservationFixture {
                id: 2,
                actor: Some(11),
                kind: SupervisorObservationKind::FaultChannel,
            },
        ],
    ),
    (
        "budget-exhaustion-escalation",
        &[
            ObservationFixture {
                id: 1,
                actor: Some(11),
                kind: SupervisorObservationKind::Restart,
            },
            ObservationFixture {
                id: 2,
                actor: None,
                kind: SupervisorObservationKind::Escalate,
            },
        ],
    ),
    (
        "parent-termination",
        &[
            ObservationFixture {
                id: 1,
                actor: None,
                kind: SupervisorObservationKind::ChildSpec,
            },
            ObservationFixture {
                id: 2,
                actor: Some(11),
                kind: SupervisorObservationKind::Stop,
            },
        ],
    ),
    (
        "state-restore-failure",
        &[
            ObservationFixture {
                id: 1,
                actor: Some(11),
                kind: SupervisorObservationKind::StateRestore,
            },
            ObservationFixture {
                id: 2,
                actor: Some(11),
                kind: SupervisorObservationKind::FaultChannel,
            },
        ],
    ),
    (
        "mailbox-cleanup",
        &[
            ObservationFixture {
                id: 1,
                actor: Some(11),
                kind: SupervisorObservationKind::Temporary,
            },
            ObservationFixture {
                id: 2,
                actor: Some(11),
                kind: SupervisorObservationKind::Stop,
            },
        ],
    ),
    (
        "vocabulary-only",
        &[
            ObservationFixture {
                id: 1,
                actor: None,
                kind: SupervisorObservationKind::OneForOne,
            },
            ObservationFixture {
                id: 2,
                actor: None,
                kind: SupervisorObservationKind::RestForOne,
            },
            ObservationFixture {
                id: 3,
                actor: None,
                kind: SupervisorObservationKind::Transient,
            },
            ObservationFixture {
                id: 4,
                actor: None,
                kind: SupervisorObservationKind::Permanent,
            },
        ],
    ),
];

fn model(fixtures: &[ObservationFixture]) -> SupervisorObservationModel {
    SupervisorObservationModel::new(
        fixtures.iter().copied().map(|fixture| {
            SupervisorObservation::new(SupervisorObservationSpec {
                id: SupervisorObservationId::new(fixture.id),
                actor: fixture.actor.map(ActorId::new),
                kind: fixture.kind,
                source_span: None,
            })
        }),
        None,
    )
    .expect("structural supervision evidence is valid")
}

#[test]
fn future_supervision_scenarios_are_recorded_without_execution() {
    let names = SCENARIOS.iter().map(|(name, _)| *name).collect::<Vec<_>>();
    assert_eq!(
        names,
        [
            "single-child-fault",
            "multiple-child-faults",
            "fault-during-restart",
            "budget-exhaustion-escalation",
            "parent-termination",
            "state-restore-failure",
            "mailbox-cleanup",
            "vocabulary-only",
        ]
    );
    assert!(
        SCENARIOS
            .iter()
            .all(|(_, fixtures)| !model(fixtures).observations().is_empty())
    );
}

#[test]
fn scenario_evidence_is_deterministic_and_order_independent() {
    for (_, fixtures) in SCENARIOS {
        let forward = model(fixtures).canonical_bytes();
        let reverse = SupervisorObservationModel::new(
            fixtures.iter().rev().copied().map(|fixture| {
                SupervisorObservation::new(SupervisorObservationSpec {
                    id: SupervisorObservationId::new(fixture.id),
                    actor: fixture.actor.map(ActorId::new),
                    kind: fixture.kind,
                    source_span: None,
                })
            }),
            None,
        )
        .expect("reordered structural evidence is valid")
        .canonical_bytes();
        assert_eq!(forward, reverse);
    }
}

#[test]
fn vocabulary_is_structural_and_contains_no_runtime_result() {
    let kinds = SCENARIOS
        .iter()
        .flat_map(|(_, fixtures)| fixtures.iter().map(|fixture| fixture.kind))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        kinds,
        BTreeSet::from([
            SupervisorObservationKind::ChildSpec,
            SupervisorObservationKind::Restart,
            SupervisorObservationKind::Stop,
            SupervisorObservationKind::Escalate,
            SupervisorObservationKind::OneForOne,
            SupervisorObservationKind::RestForOne,
            SupervisorObservationKind::Transient,
            SupervisorObservationKind::Permanent,
            SupervisorObservationKind::Temporary,
            SupervisorObservationKind::StateRestore,
            SupervisorObservationKind::FaultChannel,
        ])
    );
}
