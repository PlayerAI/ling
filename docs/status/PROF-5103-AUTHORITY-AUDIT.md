# PROF-5103 Authority Audit — Profile Composition

Status: BlockedSpec

Date: 2026-08-22

## Outcome

PROF-5103 proposes controlled composition of a base profile, target profile,
and mission constraints. Conflicts must produce explicit diagnostics, and the
canonical effective profile must participate in build identity and Semantic ID.

No accepted Profile schema or composition algebra exists. More importantly,
Accepted DEC-0012 fixes the Seed Definition/Body/Program ID domains and
versioned canonical bytes; it does not authorize adding profile inputs to those
domains. `GAP-SEMANTIC-HASH-LIFECYCLE-001` keeps identity upgrades, dependency
propagation, and migration Open. Implementing composition or changing Program
ID would therefore create an incompatible semantic identity protocol.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:122-124` is a
  non-normative plan fragment. It does not define profile precedence,
  inheritance, merge operators, conflict classes, target/mission scope,
  canonical serialization, or diagnostic fields.
- `docs/ROADMAP-1.0.md:145-149` lists artifact metadata and Semantic Graph/
  ID as future compatibility surfaces, while G5 depends on earlier replay,
  resource, and lowering boundaries. It does not authorize profile composition
  or an identity upgrade.
- Accepted DEC-0012 defines Seed `DefinitionId`, `BodyId`, and `ProgramId`
  inputs, domain separators, canonical bytes, and migration requirements.
  Profile data is not among the accepted Program ID inputs, and any algorithm,
  encoding, or normalization change requires a Semantic Schema/ID prefix
  upgrade with migration evidence.
- `GAP-CRITICAL-PROFILE-001` is Open and blocks PROF-5101/5102 and downstream
  G5 work. `GAP-SEMANTIC-HASH-LIFECYCLE-001` is Open and blocks identity
  lifecycle work; RFC-0004 is only a candidate. The Critical support matrix is
  unavailable and non-selectable.
- No RFC-K501/RFC-0012 composition decision, profile protocol, or Semantic ID
  migration exists. The plan's examples cannot establish a public `ling`
  profile format or identity behavior.

## Current implementation evidence

- The repository has no profile source/target/mission model, merge operators,
  precedence graph, conflict classifier, effective-profile canonicalizer,
  profile digest, or profile-aware build/Semantic ID implementation under
  `crates` or `tests`.
- Existing Semantic IDs follow DEC-0012's checked Typed Core and Semantic
  Schema inputs. No accepted extension incorporates compiler configuration,
  target, scheduler, mission constraints, or profile metadata into ProgramId.
- No rule fixes whether composition is monotone, override-based, intersection,
  capability subtraction, or constraint solving; how defaults and unknown
  fields merge; or how conflicts are localized and repaired.
- No stable diagnostic allocation or schema exists for conflicting profiles,
  incompatible targets/packages, impossible constraints, identity changes,
  or migration from a prior effective profile.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned Profile schema and composition algebra: layer identity,
   allowed fields, merge operators, precedence, defaults, unknown-field policy,
   conflict classes, and canonical ordering/bytes.
2. The precise relation between effective profile, build graph, artifact
   metadata, Semantic Graph, Definition/Body/Program IDs, cache keys, replay,
   and reproducible builds. Any identity change must use the accepted Semantic
   Schema/ID migration lifecycle.
3. Scope and authority for target profiles and mission constraints, including
   capability/effect, memory/ownership, numeric/concurrency, Device/Native,
   FFI, scheduler, bound, Fault, and verification obligations.
4. Configuration and profile-selection precedence under the accepted `ling`
   interface, deterministic conflict handling, migration/compatibility, and
   whether effective profiles are source-visible, project metadata, or build
   inputs only.
5. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and structured facts
   for merge conflicts, unsupported constraints, identity/schema upgrades,
   migration, and unavailable targets or packages.
6. Offline positive/negative, layer-order, conflict, canonical-byte, identity
   migration, cache/replay, reproducible-build, Unicode/CRLF, and differential
   fixtures across profiles and targets.

## Evidence and compatibility impact

The eventual implementation must canonicalize only accepted profile data and
must not make a profile declaration prove safety, capability, determinism, or
target support. It must preserve original UTF-8 spans and Semantic IDs, keep
machine identity separate from diagnostic text, and reject incompatible
composition rather than silently selecting an order.

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
diagnostics, schemas, Semantic IDs, source spans, CLI, dependency lock,
target/toolchain, support claim, or Unicode 17.0.0 behavior.

## Intentionally deferred

PROF-5103 implementation, profile layers/merge rules, effective-profile
canonical bytes, build/cache/Program ID integration, diagnostics, CLI/LSP/editor
integration, migration, and public protocol claims remain deferred until
RFC-0012 or an Accepted replacement, `GAP-CRITICAL-PROFILE-001`,
`GAP-SEMANTIC-HASH-LIFECYCLE-001`, and the G2/G3/G4 prerequisites are resolved
with executable offline identity and composition fixtures. No placeholder
profile or Semantic ID API is created.
