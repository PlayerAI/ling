//! Internal effect-recorder boundary evidence.
//!
//! This test-only inventory names proposed recordable boundaries. It does not
//! observe execution, serialize payloads, or install a recorder hook.

use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
enum PlannedRecordableEffect {
    Clock,
    Random,
    ExternalInput,
    NetworkReceive,
    FileDeviceRead,
    SchedulingNondeterminism,
}

impl PlannedRecordableEffect {
    const ALL: [Self; 6] = [
        Self::Clock,
        Self::Random,
        Self::ExternalInput,
        Self::NetworkReceive,
        Self::FileDeviceRead,
        Self::SchedulingNondeterminism,
    ];

    const fn rank(self) -> u8 {
        match self {
            Self::Clock => 0,
            Self::Random => 1,
            Self::ExternalInput => 2,
            Self::NetworkReceive => 3,
            Self::FileDeviceRead => 4,
            Self::SchedulingNondeterminism => 5,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct EffectBoundaryInventory {
    effects: Box<[PlannedRecordableEffect]>,
}

impl EffectBoundaryInventory {
    fn new(
        effects: impl IntoIterator<Item = PlannedRecordableEffect>,
    ) -> Result<Self, PlannedRecordableEffect> {
        let mut effects = effects.into_iter().collect::<Vec<_>>();
        effects.sort_unstable_by_key(|effect| effect.rank());
        let mut seen = BTreeSet::new();
        for effect in &effects {
            if !seen.insert(*effect) {
                return Err(*effect);
            }
        }
        Ok(Self {
            effects: effects.into_boxed_slice(),
        })
    }

    fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend_from_slice(b"ling.effect-recorder-observation/0");
        bytes.push(self.effects.len() as u8);
        bytes.extend(self.effects.iter().map(|effect| effect.rank()));
        bytes
    }
}

#[test]
fn planned_recordable_effect_inventory_is_complete_and_ordered() {
    let inventory = EffectBoundaryInventory::new(PlannedRecordableEffect::ALL)
        .expect("planned effect boundaries have no duplicates");
    assert_eq!(inventory.effects.as_ref(), &PlannedRecordableEffect::ALL);
    assert_eq!(
        inventory
            .effects
            .iter()
            .map(|effect| effect.rank())
            .collect::<Vec<_>>(),
        (0..6).collect::<Vec<_>>()
    );
}

#[test]
fn effect_boundary_evidence_is_order_independent_and_duplicate_checked() {
    let forward = EffectBoundaryInventory::new(PlannedRecordableEffect::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = EffectBoundaryInventory::new(PlannedRecordableEffect::ALL.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = EffectBoundaryInventory::new([
        PlannedRecordableEffect::Clock,
        PlannedRecordableEffect::Clock,
    ])
    .expect_err("duplicate effect boundary must be rejected");
    assert_eq!(duplicate, PlannedRecordableEffect::Clock);
}

#[test]
fn effect_boundary_evidence_has_no_recorder_or_runtime_authority() {
    let inventory = EffectBoundaryInventory::new([
        PlannedRecordableEffect::Clock,
        PlannedRecordableEffect::Random,
    ])
    .expect("bounded effect evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.effect-recorder-observation/0")
    );
    assert_eq!(inventory.effects.len(), 2);
}
