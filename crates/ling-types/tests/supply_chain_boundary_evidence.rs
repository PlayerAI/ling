use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum SupplyChainAttack {
    DependencyConfusion,
    NamespaceSpoofing,
    UnicodeConfusablePackage,
    MaliciousManifest,
    ArchiveTraversal,
    DecompressionBomb,
    SignatureKeyMismatch,
    YankedPackage,
    CompromisedPackageCache,
    BuildCapabilityEscalation,
}

impl SupplyChainAttack {
    const ALL: [Self; 10] = [
        Self::DependencyConfusion,
        Self::NamespaceSpoofing,
        Self::UnicodeConfusablePackage,
        Self::MaliciousManifest,
        Self::ArchiveTraversal,
        Self::DecompressionBomb,
        Self::SignatureKeyMismatch,
        Self::YankedPackage,
        Self::CompromisedPackageCache,
        Self::BuildCapabilityEscalation,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum AttackDisposition {
    LocalSubset,
    UnavailableProtocol,
}

impl AttackDisposition {
    const fn tag(self) -> u8 {
        match self {
            Self::LocalSubset => 0,
            Self::UnavailableProtocol => 1,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct AttackAssessment {
    attack: SupplyChainAttack,
    disposition: AttackDisposition,
}

const ASSESSMENTS: [AttackAssessment; 10] = [
    AttackAssessment {
        attack: SupplyChainAttack::DependencyConfusion,
        disposition: AttackDisposition::LocalSubset,
    },
    AttackAssessment {
        attack: SupplyChainAttack::NamespaceSpoofing,
        disposition: AttackDisposition::LocalSubset,
    },
    AttackAssessment {
        attack: SupplyChainAttack::UnicodeConfusablePackage,
        disposition: AttackDisposition::LocalSubset,
    },
    AttackAssessment {
        attack: SupplyChainAttack::MaliciousManifest,
        disposition: AttackDisposition::LocalSubset,
    },
    AttackAssessment {
        attack: SupplyChainAttack::ArchiveTraversal,
        disposition: AttackDisposition::UnavailableProtocol,
    },
    AttackAssessment {
        attack: SupplyChainAttack::DecompressionBomb,
        disposition: AttackDisposition::UnavailableProtocol,
    },
    AttackAssessment {
        attack: SupplyChainAttack::SignatureKeyMismatch,
        disposition: AttackDisposition::UnavailableProtocol,
    },
    AttackAssessment {
        attack: SupplyChainAttack::YankedPackage,
        disposition: AttackDisposition::UnavailableProtocol,
    },
    AttackAssessment {
        attack: SupplyChainAttack::CompromisedPackageCache,
        disposition: AttackDisposition::UnavailableProtocol,
    },
    AttackAssessment {
        attack: SupplyChainAttack::BuildCapabilityEscalation,
        disposition: AttackDisposition::UnavailableProtocol,
    },
];

#[derive(Clone, Debug, Eq, PartialEq)]
struct AttackInventory {
    assessments: Box<[AttackAssessment]>,
}

impl AttackInventory {
    fn new(
        assessments: impl IntoIterator<Item = AttackAssessment>,
    ) -> Result<Self, SupplyChainAttack> {
        let mut assessments = assessments.into_iter().collect::<Vec<_>>();
        assessments.sort_unstable_by_key(|assessment| assessment.attack.rank());
        let mut seen = BTreeSet::new();
        for assessment in &assessments {
            if !seen.insert(assessment.attack) {
                return Err(assessment.attack);
            }
        }
        Ok(Self {
            assessments: assessments.into_boxed_slice(),
        })
    }

    fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = b"ling.supply-chain-boundary-observation/0".to_vec();
        bytes.push(self.assessments.len() as u8);
        for assessment in &self.assessments {
            bytes.extend([assessment.attack.rank(), assessment.disposition.tag()]);
        }
        bytes
    }
}

#[test]
fn execution_plan_attack_list_is_complete_and_uniquely_assessed() {
    let inventory = AttackInventory::new(ASSESSMENTS).expect("attack names are unique");
    assert_eq!(
        inventory
            .assessments
            .iter()
            .map(|assessment| assessment.attack)
            .collect::<Vec<_>>(),
        SupplyChainAttack::ALL
    );
    assert_eq!(
        inventory
            .assessments
            .iter()
            .filter(|assessment| assessment.disposition == AttackDisposition::LocalSubset)
            .count(),
        4
    );
    assert_eq!(
        inventory
            .assessments
            .iter()
            .filter(|assessment| {
                assessment.disposition == AttackDisposition::UnavailableProtocol
            })
            .count(),
        6
    );
}

#[test]
fn attack_evidence_is_order_independent_and_duplicate_checked() {
    let forward = AttackInventory::new(ASSESSMENTS)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse = AttackInventory::new(ASSESSMENTS.into_iter().rev())
        .expect("reverse inventory")
        .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = AttackInventory::new([ASSESSMENTS[0], ASSESSMENTS[0]])
        .expect_err("duplicate attack assessment must be rejected");
    assert_eq!(duplicate, SupplyChainAttack::DependencyConfusion);
}

#[test]
fn opaque_evidence_bytes_are_not_a_public_security_protocol() {
    let bytes = AttackInventory::new(ASSESSMENTS)
        .expect("bounded attack inventory")
        .canonical_bytes();
    assert!(bytes.starts_with(b"ling.supply-chain-boundary-observation/0"));
    assert_eq!(
        bytes.len(),
        b"ling.supply-chain-boundary-observation/0".len() + 1 + (ASSESSMENTS.len() * 2)
    );
}
