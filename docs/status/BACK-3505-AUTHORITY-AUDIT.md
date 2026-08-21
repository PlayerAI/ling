# BACK-3505 Authority Audit — Reproducible Native Build

Status: `BlockedSpec`

Date: 2026-08-21

## Outcome

BACK-3505 proposes controlling the Native toolchain, target, linker,
environment, absolute paths, timestamps, build IDs, dependency lock, and
codegen options so identical declaration inputs produce byte-identical
artifacts, or a manifest explicitly records unavoidable differences. This is a
build/release contract that depends on the Native ABI, backend, artifact
format, security policy, and Semantic/target versioning.

No reproducible-build script, artifact manifest, path-remapping policy,
toolchain pin, build-ID rule, target matrix, linker integration, diagnostic, or
placeholder Native build crate is added. No external toolchain is installed or
executed.

## Normative traceability

- `docs/ling_execution_plan/07-G3-V0.3-NATIVE.md:393-407` is non-normative and
  follows the unaccepted NIR/backend/ABI tasks. It cannot define a stable
  artifact format, build identity, or release claim.
- BACK-3501 through BACK-3504 and NIR-3401 through NIR-3403 are `BlockedSpec`;
  RFC-N304/RFC-N306/RFC-0011 and the Native target/ABI authorities are absent
  or not Accepted.
- `GAP-NATIVE-BACKEND-ABI-001` leaves target tiers, layout, ABI, runtime,
  FFI, and cross-target behavior unresolved. `GAP-SEMANTIC-HASH-LIFECYCLE-001`
  separately leaves Semantic ID algorithm upgrades and artifact identity
  lifecycle unresolved; experimental semantic hashes must not be silently
  reused as a build reproducibility promise.
- `docs/SEMANTICS.md`, `docs/LANGUAGE.md`, and RFC-0001 reserve Native builds
  for later releases. Accepted Seed decisions and current Cargo/offline rules
  do not establish byte-identical Native artifacts.

## Current implementation evidence

- The workspace has no Native toolchain, target/linker matrix, codegen output,
  reproducible-build manifest, build-ID/path/timestamp policy, artifact
  comparer, or Native release protocol.
- Existing Cargo and VM builds are not claims about Native artifact
  reproducibility. Host paths, compiler/linker versions, environment variables,
  timestamps, debug data, section/symbol order, and platform defaults remain
  outside Ling semantics.
- No release security/supply-chain, license/TCB, dependency provenance, or
  offline artifact policy is registered for a Native backend.
- Changing dependency locks, invoking external toolchains, or publishing a
  target matrix now would imply unsupported build and support promises.

## Required authority before implementation

The accepted Native/build decisions must define:

1. The canonical build-input closure: source/Typed Core/NIR, profile/target
   manifest, compiler/runtime/backend/toolchain/linker versions and digests,
   codegen options, dependency lock, standard libraries, environment, and
   permitted host inputs.
2. Artifact identity and comparison: object/executable/debug/symbol contents,
   absolute-path remapping, timestamp/build-ID policy, section/symbol ordering,
   compression/archives, platform-specific differences, and a versioned
   manifest for differences that cannot be removed.
3. Target/ABI/backend/FFI/runtime compatibility, migration, cache and release
   boundaries, cross-target support tiers, security/supply-chain/license/TCB,
   and offline build requirements.
4. Separation of reproducibility evidence from Semantic IDs, source spans,
   language semantics, and performance claims; stable bilingual diagnostics for
   missing or mismatched inputs; and resource/time limits for comparison.
5. Independent verification and evidence: repeated clean builds, byte or
   manifest comparison, tampered/missing input rejection, cross-host/target
   cases, deterministic metadata, provenance, and interpreter/VM/Native
   semantic equivalence.

## Evidence and compatibility impact

The eventual implementation needs a pinned input/toolchain corpus, clean
rebuild and byte-comparison fixtures, manifest-difference cases, path/timestamp
and build-ID normalization tests, dependency/provenance/license checks,
tampered/missing artifact handling, cross-target evidence, and bounded offline
reproduction. It must preserve Unicode 17.0.0 and source-byte identity while
excluding host paths, timing, addresses, map/order noise, and unreviewed debug
text from semantic claims.

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, dependency lock, diagnostic registry, schema, Semantic ID,
source span, runtime, or Unicode behavior. It installs no toolchain, emits no
artifact, allocates no reproducibility diagnostic, and introduces no build or
provenance protocol.

## Intentionally deferred

Native toolchain/target/linker pinning, environment and path normalization,
timestamps/build IDs, codegen options, dependency/provenance policy, artifact
manifest and comparer, byte-identical claims, cross-target reproduction, and
all reproducible Native build evidence remain deferred until the Native
authority and accepted backend/artifact contracts exist.
