# STAB-6101 Authority Audit

- Task: `STAB-6101` — Support-Matrix Item Audit
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:32-49`
- Release: G6
- Status: `BlockedSpec`

## Decision

STAB-6101 is `BlockedSpec`. The G6 checklist asks for a row covering every
candidate Stable Feature/Profile/Target, Accepted RFC, parser/checker/Core,
execution engines, conformance, diagnostics, editor support, compatibility,
limitations, and release evidence. It does not identify the candidate Stable
set, define the acceptance or demotion rules, or establish that the current
draft matrix is a normative 1.0 support commitment. The G6 gate depends on all
G1-G5 exits, while many of those exits and their semantic authorities remain
blocked.

The current support matrix is explicitly `1.0-draft`; Seed features remain
Experimental, and Explore, Native, and Critical profiles are Unavailable. No
row may be promoted to Stable without accepted semantics, complete
implementation/conformance evidence, and a published compatibility decision.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:32-49` is a non-normative audit template. Its
  fields do not define a stable feature identity, required evidence polarity,
  compatibility version, demotion process, or the authority needed to change
  support claims.
- `docs/ROADMAP-1.0.md:500-573` makes G6 a release-planning block and requires
  all G1-G5 exits, Accepted RFCs, bidirectional traceability, conformance,
  protocol compatibility, deterministic/offline builds, and independent
  release evidence. It is not itself an Accepted language or protocol
  specification.
- `docs/ROADMAP-1.0.md:14-23` states that 1.0 is a verifiable supported surface,
  not a feature-count target, and forbids implementation from creating
  semantics before an Accepted RFC. A matrix audit cannot manufacture missing
  semantics or convert a roadmap item into Stable support.
- `docs/governance/support-matrix.toml:3` declares `matrix_target = "1.0-draft"`.
  The Seed feature rows are `Experimental`, while Explore, Native, and
  Critical are `Unavailable`; Native AOT and backend entries are
  `Unsupported`. These are current governance facts, not a Stable promise.
- The support matrix explicitly says that no profile selector, Native target,
  or Critical verifier exists, and that Semantic Graph, canonical identity,
  and Audit remain Experimental/Preview. `PROTO-DIAGNOSTIC-JSON`, bytecode,
  Semantic Graph, package, and other protocols are not generally Stable 1.x
  contracts; their lifecycle and compatibility rules must be accepted before
  promotion.
- `docs/IMPLEMENTATION.md:17` limits the Seed to its accepted scope and
  excludes Native, Resource/Borrow, Task/Actor/Node/Kernel, proof, and related
  future capabilities. The current implementation therefore cannot be
  treated as evidence for those profiles or targets.
- Open G1-G5 gaps include project/CLI and LSP boundaries, ownership/Native
  ABI, Kernel/device, Critical Profile/Node/Contract/Proof/Model Checking/
  Timing/Evidence, and compiler backend decisions. Their execution-plan
  rows remain `BlockedSpec`; no G6 audit can close those prerequisites.
- The accepted Seed/VM RFCs and governance registries authorize only the
  covered Experimental/Preview slices. They do not authorize a 1.x source,
  profile, target, protocol, diagnostic, editor, package, or backend Stable
  claim beyond their explicit clauses.

## Evidence in this repository

There is no completed G6 support-matrix audit artifact, Stable feature/profile/
target registry, 1.0 compatibility matrix, or release-candidate evidence set.
Existing governance fixtures validate the draft matrix and its unsupported
claims; they do not prove Stable support. The repository contains no complete
G1-G5 exit report, cross-engine/target conformance closure, protocol migration
corpus, or independent 1.0 release verification that could justify a Stable
row. No CLI, LSP, documentation, or public protocol currently claims that
STAB-6101 has frozen the support surface.

## Required authority before implementation

An accepted G6 stabilization decision must define, at minimum:

1. The candidate Stable Feature/Profile/Target inventory, stable identity and
   version, inclusion/demotion rules, explicit exclusions, and the rule that
   Unavailable/Unsupported/Experimental/Preview rows cannot be promoted by
   implementation evidence alone.
2. Accepted normative clauses for every candidate row, including parser,
   checker, Typed Core, interpreter/VM/Native/device behavior, effects,
   memory, Faults, capabilities, Unicode 17.0.0, and original UTF-8 spans.
3. Bidirectional traceability from clauses to implementation symbols,
   positive/negative/differential conformance, stable diagnostics, LSP/Zed
   behavior, limitations, and reproducible release artifacts. Missing or
   conflicting links must fail closed rather than create a Stable claim.
4. Compatibility and migration policy for source, Semantic IDs/canonical
   bytes, schemas/protocols, CLI/exit codes, packages/locks, bytecode, replay,
   ABI/FFI, profiles, targets, deprecation, and N-1 or unknown-field behavior.
5. Release evidence and independent review: Tier 1/2 platform scope,
   deterministic/offline builds, target/toolchain identity, fuzz and fault
   injection, security/TCB/licensing, cross-engine differential evidence, and
   the exact release-candidate/tag/provenance binding.
6. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and machine-readable
   schema fields for unsupported, incomplete, conflicting, incompatible, or
   unverifiable matrix rows, plus offline fixtures for every row and polarity.

## Compatibility and deferred work

This audit changes no language semantics, compiler/runtime behavior, public
protocol, diagnostic allocation, dependency, CLI, LSP route, schema, support
claim, or Semantic ID rule. It preserves the accepted `ling` CLI and `.ling`
source extension, checked Typed Core boundary, original UTF-8 spans, Unicode
17.0.0, deterministic identity rules, and the current truthful Experimental/
Preview/Unavailable/Unsupported matrix states.

It deliberately adds no Stable feature/profile/target row, compatibility
promise, release artifact, diagnostic, CLI command, editor route, protocol, or
placeholder API, and introduces no stale `zero` names. STAB-6101 remains
deferred until G1-G5 exits are complete, each candidate has Accepted authority
and executable evidence, and governance records a reviewed 1.0 matrix and
compatibility policy.
