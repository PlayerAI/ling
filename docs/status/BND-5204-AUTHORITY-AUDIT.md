# BND-5204 Authority Audit — Resource-Budget Diagnostics

Status: BlockedSpec

Date: 2026-08-22

## Outcome

BND-5204 proposes a diagnostic view containing a budget, estimated or proved
usage, largest contributors, path/provenance, unknown assumptions, and
candidate transformations. The output would depend on the memory and
boundedness facts requested by BND-5203 and on the Critical Profile policy.

No Accepted RFC defines those facts, their proof status, their machine-readable
schema, or the semantics of a candidate transformation. The current
`ling.diagnostic/0.1` JSON container is Preview and its existing Facts/Repair
compatibility boundary does not authorize inventing resource-budget meanings,
new codes, a `FixPlan` wire type, or a semantics-changing repair. Diagnostics
cannot be implemented as a presentation-only feature while their inputs and
repairs remain undefined.

## Normative traceability

- `docs/ling_execution_plan/09-G5-V0.5-CRITICAL.md:184-195` is a
  non-normative plan fragment. It names display fields but defines no types,
  units, proof/estimate states, contributor ordering, provenance identity,
  assumption model, unknown behavior, or transformation transaction.
- `docs/ROADMAP-1.0.md:243-249` requires diagnostics to reuse the compiler's
  model and requires code actions to use versioned Workspace Edits or Semantic
  Transactions. `:433-498` requires Critical boundedness and independently
  reproducible evidence, but does not authorize this diagnostic schema.
- `GAP-CRITICAL-PROFILE-001` keeps Critical capabilities, boundedness, and
  evidence claims Open. BND-5203 is itself blocked by that gap and by the
  ownership, mailbox/concurrency, Native/ABI, and Kernel/Device gaps.
- `GAP-LSP-TRANSACTION-PROTOCOL-001` and
  `GAP-SEMANTIC-PROTOCOL-LIFECYCLE-001` leave snapshot/version preconditions,
  stale-edit rejection, Stable versus Experimental fields, and schema
  migration unresolved. `GAP-PROJECT-CLI-INTERFACE-001` also leaves project
  CLI/report behavior unresolved.
- DEC-0001/DEC-0002 and the protocol inventory authorize the existing
  bilingual diagnostic code registry and Preview `ling.diagnostic/0.1`
  container. They do not allocate a budget domain or establish the meaning of
  a resource contribution, proof state, assumption, or repair.
- The accepted diagnostic boundary requires existing codes' meanings and
  Facts types to remain compatible; a new root-cause meaning requires a
  registered code and accepted evidence. No such BND diagnostic allocation is
  present in `docs/ERROR-CODES.md`.

## Current implementation evidence

- The compiler has no resource-budget fact producer, memory/termination proof
  model, contributor ranking, path/provenance engine, or Critical Profile
  checker under `crates` or `tests`.
- `ling-diagnostics` can render the existing registered bilingual diagnostics
  and structured Facts/Repairs, but it has no accepted resource-budget fields,
  budget-specific code, or transformation protocol.
- Existing VM `resource_limit` and `out_of_memory` Runtime Faults describe
  host-safety behavior for a verified bytecode execution. They do not provide
  source-level budget facts, contributor attribution, proof status, or a
  candidate source transformation.
- No rule fixes deterministic ordering for largest contributors, control-flow
  paths, assumptions, target/compiler provenance, or multiple budget failures.
  No rule separates an estimate, a proof, a runtime guard, an unknown, and an
  unsupported profile claim.
- No accepted CLI, LSP, Workspace Edit, Semantic Transaction, confirmation,
  rollback, or migration contract can safely publish a candidate transformation.
  The current public command name remains `ling`; no stale plan spelling is
  introduced by this audit.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. The BND-5203 memory and boundedness fact model, including units, target and
   compiler identity, proof/estimate/assumption/unknown states, source spans,
   Semantic IDs, path selection, contributor ordering, and deterministic
   serialization.
2. A versioned diagnostic schema for `budget`, usage, contributors,
   provenance, assumptions, and unsupported/overflow/target-mismatch states,
   including required and optional field types, limits, localization, and
   compatibility migration.
3. Stable bilingual `L-<DOMAIN>-<NUMBER>` allocations and structured Facts
   for exceeded, unknown, estimated, unproved, unavailable, target-mismatch,
   and runtime-fallback cases; the code registry must remain the sole
   allocation source.
4. A precise distinction between a diagnostic Repair candidate and a
   semantics-changing transformation, with preconditions, equivalence proof,
   ownership/effect/resource preservation, source-map preservation, user
   consent, rollback, and failure behavior.
5. Accepted `ling` CLI, LSP, Workspace Edit, and Semantic Transaction
   boundaries for publishing facts and repairs, including snapshot/version
   checks, stale-result rejection, cancellation, ordering, limits, and
   machine-readable output.
6. Offline positive/negative, boundary, unknown-assumption, target/compiler,
   provenance, localization, Unicode 17, determinism, migration,
   stale-transaction, repair-equivalence, and differential fixtures with
   bounded diagnostic size and resource use.

## Evidence and compatibility impact

The eventual implementation must consume checked Typed Core and publish only
facts authorized by the accepted budget/profile model. It must keep machine
fields separate from localized text, preserve original UTF-8 byte spans and
Semantic IDs, and avoid exposing host paths, addresses, allocator text,
timing, hash order, or debug output. A candidate transformation must never be
silently applied or presented as proof.

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory or ownership behavior,
diagnostic code, diagnostic schema, protocol inventory, Semantic IDs, source
spans, CLI, LSP, dependency lock, target/toolchain support claim, or Unicode
17.0.0 behavior.

## Intentionally deferred

BND-5204 implementation, resource-budget facts and diagnostics, new error-code
allocations, CLI/LSP routes, Repair or transformation schemas, and public
support claims remain deferred until RFC-K504 (or an Accepted replacement),
BND-5203, `GAP-CRITICAL-PROFILE-001`, the ownership/concurrency/Native/Device
authorities, and the CLI/LSP/Semantic Transaction gaps are resolved with
independent offline fixtures. No placeholder diagnostic field, code,
`FixPlan`, or public API is created.
