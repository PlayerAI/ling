//! Internal determinism-class evidence.
//!
//! The classes below are planning vocabulary only. This test corpus does not
//! classify programs, write build metadata, or create a replay header.

use ling_effects::{EffectLabel, EffectRowModel};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedDeterminismClass {
    Strict,
    Seeded,
    RecordedEffects,
    BestEffort,
}

impl PlannedDeterminismClass {
    const ALL: [Self; 4] = [
        Self::Strict,
        Self::Seeded,
        Self::RecordedEffects,
        Self::BestEffort,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::Strict => 0,
            Self::Seeded => 1,
            Self::RecordedEffects => 2,
            Self::BestEffort => 3,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DeterminismEvidence {
    class: PlannedDeterminismClass,
    effects: EffectRowModel,
}

impl DeterminismEvidence {
    fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"ling.determinism-observation/0");
        bytes.push(self.class.rank());
        bytes.extend_from_slice(&self.effects.canonical_bytes());
        bytes
    }
}

fn effect_row() -> EffectRowModel {
    EffectRowModel::closed([
        EffectLabel::clock(),
        EffectLabel::random(),
        EffectLabel::console_write(),
    ])
}

#[test]
fn planned_class_vocabulary_is_complete_and_ordered() {
    assert_eq!(
        PlannedDeterminismClass::ALL,
        [
            PlannedDeterminismClass::Strict,
            PlannedDeterminismClass::Seeded,
            PlannedDeterminismClass::RecordedEffects,
            PlannedDeterminismClass::BestEffort,
        ]
    );
    assert_eq!(
        PlannedDeterminismClass::ALL
            .iter()
            .map(|class| class.rank())
            .collect::<Vec<_>>(),
        [0, 1, 2, 3]
    );
}

#[test]
fn class_evidence_is_deterministic_without_runtime_classification() {
    let first = DeterminismEvidence {
        class: PlannedDeterminismClass::RecordedEffects,
        effects: effect_row(),
    };
    let second = DeterminismEvidence {
        class: PlannedDeterminismClass::RecordedEffects,
        effects: EffectRowModel::closed([
            EffectLabel::console_write(),
            EffectLabel::random(),
            EffectLabel::clock(),
        ]),
    };
    assert_eq!(first.canonical_bytes(), second.canonical_bytes());
}

#[test]
fn all_planned_classes_share_only_checked_effect_evidence() {
    let rows = PlannedDeterminismClass::ALL
        .into_iter()
        .map(|class| DeterminismEvidence {
            class,
            effects: effect_row(),
        })
        .collect::<Vec<_>>();
    assert!(
        rows.iter()
            .all(|evidence| !evidence.effects.canonical_bytes().is_empty())
    );
    assert!(rows.iter().all(|evidence| {
        evidence
            .canonical_bytes()
            .starts_with(b"ling.determinism-observation/0")
    }));
}
