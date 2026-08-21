# TIM-5702 Authority Audit

- Task: `TIM-5702` — Measurement and Static-Analysis Separation
- Plan: `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:489-501`
- Release: G5
- Status: `BlockedSpec`

## Decision

TIM-5702 is `BlockedSpec`. The plan proposes five report labels—`Measured`,
`Estimated`, `StaticallyBounded`, `Assumed`, and `Unknown`—and correctly warns
that an observed average or maximum is not a WCET proof. It does not define
their meanings, transitions, evidence requirements, target model, or report
schema. The labels are therefore a planning vocabulary, not an accepted public
status model.

No accepted specification authorizes a timing measurement pipeline, static
bound checker, WCET claim, or common evidence representation. Implementing the
labels or accepting measured maxima as timing facts would invent target and
evidence semantics beyond the Seed subset and could turn empirical data into a
false safety claim.

## Normative traceability

- `09-G5-V0.5-CRITICAL.md:489-501` is a non-normative checklist. It has no
  status enum, provenance fields, confidence or uncertainty rules, sampling
  model, static-analysis soundness boundary, or migration/compatibility policy.
- `docs/SEMANTICS.md:1215-1238` sketches Contract evidence states such as
  `Proved`, `RuntimeChecked`, `ModelChecked`, `Tested`, `Assumed`, and
  `Unverified`, but `SEMANTICS` is Draft and those states describe Contract
  claims, not timing measurements or WCET. They cannot be silently reused as a
  timing protocol.
- `docs/SEMANTICS.md:1385-1425` and `docs/ROADMAP-1.0.md:439-466` describe
  Node clocks, WCET/deadline assumptions, and future Critical evidence gates;
  `LANGUAGE` and `ROADMAP-1.0` remain Draft/Planning authorities. They do not
  fix a target, clock, instrumentation, static analyzer, or report format.
- `GAP-CRITICAL-PROFILE-001` remains Open and explicitly leaves Node timing,
  boundedness, Critical claims, and the evidence schema unaccepted. Its
  candidate RFC-0012 is not present or Accepted.
- `PROTO-EVIDENCE` is Planned public/Future, unimplemented, unversioned,
  non-canonical, schema-free, and fixture-free. Its reader, writer,
  verification, identity, provenance, and migration rules are expressly not
  defined. No timing protocol appears elsewhere in the protocol inventory.
- Accepted RFC-0014 and RFC-0020 provide deterministic VM resource and host
  robustness evidence, not target measurements, static WCET bounds, or a
  measured-versus-estimated status model. RFC-0019 compares logical
  interpreter/VM outcomes and excludes instruction counts and other host
  details. DEC-0019's scheduler is only an internal compiler-query boundary.

## Evidence in this repository

There is no timing measurement harness, instrumentation contract, target clock
model, static WCET analyzer, status enum, report reader/writer, evidence
verifier, or timing fixture under `crates/` or `tests/`. Existing VM step/frame/
heap tests and compiler query determinism tests have different accepted scopes.
No `ling` CLI, LSP request, diagnostic, or public protocol claims TIM-5702
support.

## Required authority before implementation

An accepted RFC or replacement decision must define, at minimum:

1. A versioned canonical timing-result schema and closed status vocabulary,
   including exact semantics and allowed transitions for measured, estimated,
   statically bounded, assumed, unknown, invalid, and unsupported outcomes.
2. The separation boundary between instrumentation/measurement, estimation,
   static analysis, proof, and assumption. It must specify soundness claims,
   confidence/uncertainty, sample and aggregation rules, calibration, clock
   behavior, interference, and why an observed maximum is not automatically a
   WCET bound.
3. Target, profile, compiler/toolchain, build, scheduler, interrupt,
   cache/memory, device/FFI, input, and environment identity, plus the TCB and
   reproducibility requirements for each conclusion. Host paths, addresses,
   wall-clock text, allocator layout, and debug output must not become Ling
   identity.
4. Source and Semantic ID linkage to Timing IR/path records with original
   UTF-8 byte spans, and explicit handling for optimized-away code, unknown
   paths, unbounded loops, missing assumptions, instrumentation perturbation,
   and target mismatch.
5. Independent evidence verification, schema migration, malformed/contradictory
   records, unknown fields, and fail-closed behavior, with registered bilingual
   `L-<DOMAIN>-<NUMBER>` diagnostics and documented process/fixture outcomes.
6. Offline positive, negative, boundary, calibration, target/profile variation,
   migration, Unicode 17.0.0, BOM/CRLF, source-span, repeated-run
   determinism, and differential fixtures. Fixtures must preserve the
   distinction between empirical observations and proofs.

## Compatibility and deferred work

This audit changes no language semantics, compiler/runtime behavior, public
protocol, diagnostic allocation, dependency, CLI, LSP route, schema, or
support claim. It preserves the accepted `ling` CLI and `.ling` source
extension, original UTF-8 spans, Unicode 17.0.0, deterministic identity rules,
and the checked Typed Core boundary. It deliberately adds no timing status
enum, measurement API, analyzer, evidence writer/verifier, deadline hook, or
placeholder API, and it introduces no stale `zero` names.

TIM-5702 remains deferred until the Timing IR, Critical Profile, Node,
boundedness, target/ABI, scheduler, device/FFI, Contract/Proof, and evidence
authorities are Accepted with executable fixtures. TIM-5703 must not consume
these labels as if they were already a WCET or deadline contract.
