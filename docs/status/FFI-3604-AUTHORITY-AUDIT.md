# FFI-3604 Authority Audit — Target Primitive Package

Status: `BlockedSpec`

Date: 2026-08-21

## Outcome

FFI-3604 proposes a trusted target package containing `package.toml`,
`primitives.lingabi`, implementation files, proof/tests, capabilities, and a
TCB declaration. It also proposes that ordinary Ling packages cannot declare
arbitrary primitives and that explicit build configuration admits a trusted
package into the TCB. The proposal is an execution-plan sketch, not an
accepted target, package, capability, or TCB protocol.

No `targets/<target>` package, `lingabi` schema, target manifest, primitive
loader, capability policy, TCB evaluator, target selector, proof/test
verifier, build integration, or public package API is added. The v0.0.1 Seed
compiler and its interpreter/VM behavior remain unchanged while Native ABI,
ownership, FFI, target, and evidence decisions are unresolved.

## Normative traceability

- `docs/ling_execution_plan/07-G3-V0.3-NATIVE.md:460-474` is non-normative.
  Its directory is a proposal and does not define package identity, target
  selection, `lingabi` fields, primitive signatures, capability semantics,
  proof acceptance, TCB membership, or update/revocation rules.
- `docs/SEMANTICS.md:49-70` reserves `TargetManifest` and target primitives
  in the future semantic snapshot, while `:1831-1868` requires signed FFI
  boundaries, trusted Target Packages, and an explicit TCB. These clauses do
  not accept a package format or executable primitive.
- `docs/LANGUAGE.md:669-683` and `:1276-1287` place unsafe hardware/ABI work in
  an independent Target Primitive Package and require explicit ABI, ownership,
  thread/reentry, Error/Fault, Capability, and package boundaries. They do
  not define package loading, trust, or target compatibility.
- `docs/SEMANTICS.md:1872-1931` excludes Native backend and Critical
  enforcement from the v0.0.1 formal subset; `docs/governance/support-matrix.toml:175-183`
  records no committed Ling Native target and marks `TARGET-NATIVE-AOT`
  `Unsupported`, blocked by the Native ABI and ownership gaps.
- `GAP-NATIVE-BACKEND-ABI-001` remains Open and explicitly blocks FFI-3604;
  target tiers, ABI/layout, Typed FFI, Fault/unwinding, thread/reentry, and
  Target Primitive behavior are unaccepted. Its next action is RFC-0011 after
  RFC-0007 defines the exposed memory categories.
- `docs/governance/protocol-inventory.toml` has no implemented `lingabi` or
  target-package protocol. `PROTO-ABI` and `PROTO-EVIDENCE` are Planned public
  with no schema, version, migration, reader, writer, or fixtures.
- RFC-N304, RFC-N305, RFC-N306, RFC-0007, and RFC-0011 are not Accepted
  authorities in this repository; RFC-0001 remains Draft under DEC-0018.

## Current implementation evidence

- The workspace has no committed Native target, target package directory,
  `lingabi` reader/verifier, target manifest schema, primitive registry,
  capability evaluator, TCB manifest/checker, target selector, or target
  backend. Existing `Console.Write` is a Seed primitive and is not a model
  for arbitrary trusted hardware/ABI packages.
- No accepted rule establishes package identity, dependency and lock
  interaction, target/profile selection, capability grant/deny semantics,
  proof status, hardware assumptions, revocation, or whether implementation
  artifacts are source, binary, or generated. Host platform behavior cannot
  supply those missing Ling semantics.
- No target toolchain, hardware dependency, generated package, unsafe surface,
  diagnostic allocation, or public protocol implementation is required for
  this audit.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned target-package and `lingabi` schema: package/target identity,
   primitive signatures and layouts, ABI/target/profile constraints,
   capability requirements, source and Semantic-ID provenance, dependencies,
   lock entries, artifact identity, unknown-field and migration policy, and
   deterministic canonical bytes.
2. Trust and TCB rules: package ownership/signing or other admission
   authority, compiler/proof-kernel/backend/runtime/hardware assumptions,
   proof/test status, implementation language and unsafe boundary, license
   and supply-chain provenance, revocation/update compatibility, and explicit
   exclusion of ordinary packages, AI, IDE, and Formatter from implicit trust.
3. Primitive semantics and safety: ownership/borrow/Resource/Managed and
   lifetime behavior, layout and calling convention, thread/reentry/blocking,
   Error/Fault/unwind, Capability/Effect/Profile, bounds/aliasing, and target
   availability/rejection rules, as consumed by verified Typed Core and the
   Native backend.
4. Build and release integration: hermetic/offline inputs, target/toolchain
   selection, generated/shim provenance and build-hash rules, cross-target
   compatibility, cache/release identity, reproducibility, tamper detection,
   and independent package/ABI/evidence readers.
5. Stable bilingual diagnostics and executable positive/negative fixtures for
   package discovery, malformed metadata, unknown targets/capabilities,
   untrusted packages, proof/TCB failures, version migration, and every
   unsupported primitive boundary.

## Evidence and compatibility impact

The eventual implementation needs target-package and `lingabi` schema golden
corpus; package identity/lock/dependency and target/profile selection tests;
primitive signature/layout/ABI and capability checks; proof/TCB/admission and
license/provenance evidence; malformed, tampered, unknown-field, revoked, and
migration fixtures; cross-target and Native/FFI differential tests;
deterministic clean/offline/reproducible builds; sanitizer/fuzz coverage; and
independent readers. It must preserve original UTF-8 byte spans, stable
Semantic IDs, bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and Unicode
17.0.0 behavior without exposing host paths, addresses, filesystem order,
Rust layout, or hardware details as Ling semantics.

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, dependency lock, memory category, Managed/Resource/ownership
behavior, diagnostic registry, schema, Semantic ID, source span, runtime,
support matrix, target, or Unicode behavior. It adds no target package,
`lingabi` schema, primitive registry, capability/TCB checker, dependency,
toolchain, diagnostic, public protocol implementation, or placeholder API.

## Intentionally deferred

Target package and `lingabi` schemas, package discovery/locking, target/profile
selection, primitive lowering, capability and TCB admission, proof/test
verification, ABI/ownership/runtime integration, provenance/license/revocation,
build/release and cross-target support, protocol readers/migrations, fixtures,
sanitizer/fuzz/differential evidence, and all Native/Target Primitive claims
remain deferred until RFC-N305 and the dependent ownership, Native ABI,
target, runtime, `PROTO-ABI`, and `PROTO-EVIDENCE` authorities are Accepted.
