# MC-5603 Authority Audit

Task: `MC-5603` — Model-Check Report Semantics
Release: G5
Status: `BlockedSpec`

## Outcome

`MC-5603` is not implementable from the current accepted authority. The
execution plan proposes four report states: `CounterexampleFound`,
`NoCounterexampleWithinBounds`, `Inconclusive`, and `InvalidModel`, and
correctly says that bounded absence must not be written as “the system is
safe.” These labels do not define a versioned report schema, validity rules,
bound/assumption disclosure, counterexample identity, error/exit semantics,
or the relationship to proof and evidence.

`MC-5601` and `MC-5602` are `BlockedSpec`; RFC-K506 is absent, and the Node,
Task/Actor, boundedness, Contract/proof, Critical profile, replay, and evidence
authorities remain unresolved. `PROTO-EVIDENCE` is Future without a schema or
fixtures. No report model, result enum, counterexample payload, diagnostic,
CLI command, public protocol, or placeholder API should be added.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:458-469` is a
  non-normative checklist. It names report states and one non-claim, but does
  not define required fields, canonical encoding, versioning, state
  transitions, evidence linkage, or machine-readable compatibility.
- The plan assigns finite-state projection and model-checking semantics to
  absent RFC-K506; RFC-K501/K502/K504 define missing Critical, Node, and bound
  inputs, while RFC-K505/K507 define missing proof and evidence boundaries.
- `docs/SEMANTICS.md:1214-1224` is Draft. `ModelChecked(model_id, bound)` is
  only a status sketch and does not authorize a public report or prove a
  bounded result. `docs/LANGUAGE.md` and `docs/ROADMAP-1.0.md` are likewise
  below accepted implementation authority for this surface.
- `docs/governance/gap-register.toml` records open
  `GAP-CRITICAL-PROFILE-001`, explicitly leaving model-check claims,
  boundedness, Contract proof/runtime, and evidence schema unaccepted.
- `docs/governance/protocol-inventory.toml` records `PROTO-EVIDENCE` as
  Planned public/Future with no current version, schema, canonical form,
  reader/writer policy, migration tool, or fixtures. No model-check report
  protocol is inventoried.
- Accepted RFC-0019 compares bounded logical interpreter/VM outcomes for
  executable Seed fixtures. It is not a model-check report and does not define
  the proposed result states, bounds/assumptions disclosure, or
  counterexample semantics.
- Existing bilingual diagnostic JSON and CLI exit behavior are accepted only
  for current compiler/runtime failures; they do not authorize model-check
  report states or a new public command.

## Repository evidence

The repository has no model-check report type, result enum, bounded-result
schema, counterexample payload, report reader/writer, model-check diagnostic
codes, or report fixtures. Existing semantic snapshots and differential
reports describe checked Seed programs and VM outcomes, not explored state
spaces. No implementation can currently distinguish an invalid projection
from an invalid property, a timeout from an exhausted bound, or an
unverifiable counterexample under a stable public contract.

## Required authority before implementation

An accepted RFC-K506/RFC-K507 replacement coordinated with MC-5601/MC-5602,
Node/Task/Actor, bounds, Contract/proof, replay, and Critical decisions must
define at least:

1. A versioned canonical report schema with result states, model/property/bound
   identity, source and Semantic IDs/spans, assumptions, scheduler/time
   configuration, explored-state/transition counts, resource limits and
   exhaustion reason, tool/version identity, provenance, checksums/signatures,
   redaction, unknown fields, and migration rules.
2. Precise semantics for counterexample found, no counterexample within the
   declared bounds, inconclusive/timeout/memory exhaustion, invalid model or
   property, malformed/corrupt input, unsupported version, and unknown
   results. Define exit codes and fail-closed behavior; never imply global
   safety from bounded absence.
3. Counterexample and replay linkage, proof/assumption/evidence distinction,
   independent verification requirements, stable bilingual
   `L-<DOMAIN>-<NUMBER>` diagnostics, deterministic ordering, and rules
   keeping host paths, timing, addresses, allocation, and debug output out of
   Ling identity.
4. Offline positive/negative, each result state, bound-edge, timeout/memory,
   invalid model/property, malformed/corrupt, counterexample/replay,
   migration, Unicode 17.0.0, CRLF/BOM/source-span, deterministic repeated
   run, and checker/differential fixtures before any report support claim.

## Compatibility and deferred work

This audit changes no parser, resolver, AST/HIR, Checked Typed Core,
evaluator, bytecode, VM, scheduler, mailbox, diagnostics, schema, CLI, LSP,
dependency, Semantic ID, or public protocol. It preserves checked-only
execution, accepted Seed semantics, original UTF-8 byte spans, Unicode
17.0.0, deterministic ordering, and exclusion of host paths, timing,
addresses, and debug output from Ling identity. It makes no model-check,
report, counterexample, bounded-safety, proof, or Critical support claim.

Implementation remains deferred until RFC-K506/RFC-K507 or accepted
replacements and executable projection/exploration/replay/evidence fixtures
define the report boundary. Do not add a report enum, schema, counterexample
payload, diagnostic allocation, CLI/LSP route, public protocol, support claim,
or placeholder API while those authorities remain unresolved.
