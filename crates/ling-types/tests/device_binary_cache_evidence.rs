use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
enum PlannedDeviceBinaryCacheBoundary {
    DeviceBinaryCache,
    ProgramId,
    SemanticId,
    DeviceIrVersion,
    BackendVersion,
    TargetArchitecture,
    RuntimeCompatibility,
    DriverCompatibility,
    NumericMode,
    Profile,
    CompilerOptions,
    CacheNamespace,
    CanonicalArtifact,
    CacheKey,
    IdentitySeparation,
    TypedCoreValidation,
    DeviceIrValidation,
    BackendCapability,
    Signature,
    Verification,
    TrustBoundary,
    Permissions,
    PathIsolation,
    AtomicPublish,
    ConcurrentWriters,
    Eviction,
    DiskLimit,
    Disposable,
    PortableCache,
    CacheHit,
    CacheMiss,
    Corruption,
    UnknownVersion,
    AbiMismatch,
    CapabilityMismatch,
    EnvironmentChange,
    StaleOptions,
    RecompileFallback,
    Migration,
    Privacy,
    PathExclusion,
    AddressExclusion,
    TimestampExclusion,
    AllocationOrderExclusion,
    DriverTextExclusion,
    DebugOutputExclusion,
    PositiveFixture,
    NegativeFixture,
    CorruptionFixture,
    MigrationFixture,
    CrossToolchainFixture,
    CrossTargetFixture,
    NumericProfileFixture,
    SecurityFixture,
    ReplayFixture,
    DifferentialFixture,
    UnicodeFixture,
    DeterminismFixture,
    DiagnosticCode,
    ProtocolInventory,
}

impl PlannedDeviceBinaryCacheBoundary {
    const ALL: [Self; 60] = [
        Self::DeviceBinaryCache,
        Self::ProgramId,
        Self::SemanticId,
        Self::DeviceIrVersion,
        Self::BackendVersion,
        Self::TargetArchitecture,
        Self::RuntimeCompatibility,
        Self::DriverCompatibility,
        Self::NumericMode,
        Self::Profile,
        Self::CompilerOptions,
        Self::CacheNamespace,
        Self::CanonicalArtifact,
        Self::CacheKey,
        Self::IdentitySeparation,
        Self::TypedCoreValidation,
        Self::DeviceIrValidation,
        Self::BackendCapability,
        Self::Signature,
        Self::Verification,
        Self::TrustBoundary,
        Self::Permissions,
        Self::PathIsolation,
        Self::AtomicPublish,
        Self::ConcurrentWriters,
        Self::Eviction,
        Self::DiskLimit,
        Self::Disposable,
        Self::PortableCache,
        Self::CacheHit,
        Self::CacheMiss,
        Self::Corruption,
        Self::UnknownVersion,
        Self::AbiMismatch,
        Self::CapabilityMismatch,
        Self::EnvironmentChange,
        Self::StaleOptions,
        Self::RecompileFallback,
        Self::Migration,
        Self::Privacy,
        Self::PathExclusion,
        Self::AddressExclusion,
        Self::TimestampExclusion,
        Self::AllocationOrderExclusion,
        Self::DriverTextExclusion,
        Self::DebugOutputExclusion,
        Self::PositiveFixture,
        Self::NegativeFixture,
        Self::CorruptionFixture,
        Self::MigrationFixture,
        Self::CrossToolchainFixture,
        Self::CrossTargetFixture,
        Self::NumericProfileFixture,
        Self::SecurityFixture,
        Self::ReplayFixture,
        Self::DifferentialFixture,
        Self::UnicodeFixture,
        Self::DeterminismFixture,
        Self::DiagnosticCode,
        Self::ProtocolInventory,
    ];

    const fn rank(self) -> u8 {
        self as u8
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct DeviceBinaryCacheInventory {
    boundaries: Box<[PlannedDeviceBinaryCacheBoundary]>,
}

impl DeviceBinaryCacheInventory {
    fn new(
        boundaries: impl IntoIterator<Item = PlannedDeviceBinaryCacheBoundary>,
    ) -> Result<Self, PlannedDeviceBinaryCacheBoundary> {
        let mut boundaries = boundaries.into_iter().collect::<Vec<_>>();
        boundaries.sort_unstable_by_key(|boundary| boundary.rank());
        let mut seen = BTreeSet::new();
        for boundary in &boundaries {
            if !seen.insert(*boundary) {
                return Err(*boundary);
            }
        }
        Ok(Self {
            boundaries: boundaries.into_boxed_slice(),
        })
    }

    fn canonical_bytes(&self) -> Vec<u8> {
        let mut bytes = b"ling.device-binary-cache-observation/0".to_vec();
        bytes.push(self.boundaries.len() as u8);
        bytes.extend(self.boundaries.iter().map(|boundary| boundary.rank()));
        bytes
    }
}

#[test]
fn proposed_device_binary_cache_boundaries_are_complete_and_ordered() {
    let inventory = DeviceBinaryCacheInventory::new(PlannedDeviceBinaryCacheBoundary::ALL)
        .expect("planned device binary cache boundaries have no duplicates");
    assert_eq!(
        inventory.boundaries.as_ref(),
        &PlannedDeviceBinaryCacheBoundary::ALL
    );
    assert_eq!(
        inventory
            .boundaries
            .iter()
            .map(|boundary| boundary.rank())
            .collect::<Vec<_>>(),
        (0..60).collect::<Vec<_>>()
    );
}

#[test]
fn device_binary_cache_evidence_is_order_independent_and_duplicate_checked() {
    let forward = DeviceBinaryCacheInventory::new(PlannedDeviceBinaryCacheBoundary::ALL)
        .expect("forward inventory")
        .canonical_bytes();
    let reverse =
        DeviceBinaryCacheInventory::new(PlannedDeviceBinaryCacheBoundary::ALL.into_iter().rev())
            .expect("reverse inventory")
            .canonical_bytes();
    assert_eq!(forward, reverse);

    let duplicate = DeviceBinaryCacheInventory::new([
        PlannedDeviceBinaryCacheBoundary::DeviceBinaryCache,
        PlannedDeviceBinaryCacheBoundary::DeviceBinaryCache,
    ])
    .expect_err("duplicate device binary cache boundary must be rejected");
    assert_eq!(
        duplicate,
        PlannedDeviceBinaryCacheBoundary::DeviceBinaryCache
    );
}

#[test]
fn device_binary_cache_evidence_has_no_cache_authority() {
    let inventory = DeviceBinaryCacheInventory::new([
        PlannedDeviceBinaryCacheBoundary::DeviceBinaryCache,
        PlannedDeviceBinaryCacheBoundary::ProgramId,
        PlannedDeviceBinaryCacheBoundary::SemanticId,
        PlannedDeviceBinaryCacheBoundary::CacheKey,
        PlannedDeviceBinaryCacheBoundary::RecompileFallback,
        PlannedDeviceBinaryCacheBoundary::Disposable,
        PlannedDeviceBinaryCacheBoundary::IdentitySeparation,
        PlannedDeviceBinaryCacheBoundary::ProtocolInventory,
    ])
    .expect("bounded device binary cache evidence");
    assert!(
        inventory
            .canonical_bytes()
            .starts_with(b"ling.device-binary-cache-observation/0")
    );
    assert_eq!(inventory.boundaries.len(), 8);
}
