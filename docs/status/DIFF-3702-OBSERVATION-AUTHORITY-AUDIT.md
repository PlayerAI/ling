# DIFF-3702-OBSERVATION Authority Audit — Allowed-Difference Boundary Evidence

Status: `BlockedSpec`

Date: 2026-08-23

## Outcome

DIFF-3702-OBSERVATION is limited to test-local vocabulary for a future
allowed-difference registry. It does not decide which observations are
unobservable, add a registry, or permit any Interpreter/VM/Native difference.
Public DIFF-3702 remains `BlockedSpec`.

## Normative traceability

- `docs/ling_execution_plan/07-G3-V0.3-NATIVE.md:512-522` is non-normative.
  It proposes candidate differences but does not define registry fields,
  authority, scope, predicates, expiry, ownership, migration, or conflict
  behavior.
- `docs/ROADMAP-1.0.md:70-78` requires canonical output to exclude host paths,
  HashMap order, Rust debug text, addresses, and thread scheduling details,
  while `:351-379` requires Native and differential evidence. These clauses do
  not authorize suppressing numeric, cleanup, replay, FFI, or target behavior.
- Accepted RFC-0018/RFC-0019 and DEC-0142 govern only the current experimental
  checked Interpreter–VM boundary. They do not establish a Native or
  allowed-difference equivalence contract.
- `GAP-NATIVE-BACKEND-ABI-001`, `GAP-NUMERIC-CHECKED-FAULT-001`,
  `GAP-DETERMINISTIC-REPLAY-001`, and `GAP-SEMANTIC-HASH-LIFECYCLE-001`
  remain Open. `PROTO-ABI`, `PROTO-EVIDENCE`, and Native/FFI/ownership RFCs are
  not Accepted authorities for a registry.

## Current implementation evidence

- The workspace has no allowed-difference registry, schema, reader, entry,
  backend exemption, comparison predicate, or three-engine harness.
- The new test records sixty provisional boundary labels, explicit local rank,
  duplicate rejection, and insertion-order-independent opaque bytes only.
- No accepted rule defines performance/address/timing/allocation observability,
  GC/cleanup or BestEffort scheduling, numeric/NaN tolerance, replay/effect-log
  equivalence, FFI/target variation, or registry provenance and migration.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned, independently readable registry schema with stable entry
   identity, authority/source, engine/target/profile scope, observable field,
   comparison predicate, rationale, owner, status, review/expiry, migration,
   provenance, and positive/negative fixtures. Missing or unknown entries must
   fail closed.
2. Semantic classification for every candidate difference. Values, events,
   Faults, effects, capabilities, source spans, Semantic IDs, and selected
   Resource/Managed observations must not be suppressed merely because a
   backend differs.
3. Accepted numeric, replay, concurrency, cleanup, FFI, ABI, ownership, and
   target contracts that bound predicates and divergence.
4. DIFF-3701 and `PROTO-ABI`/`PROTO-EVIDENCE` integration with deterministic
   cross-process/offline provenance and schema migration.
5. Stable bilingual `L-<DOMAIN>-<NUMBER>` diagnostics for unknown, expired,
   overlapping, contradictory, unauthorized, and out-of-scope entries, plus
   release gates proving every exception is specification-backed.

## Compatibility and intentionally deferred work

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, dependency lock, memory category, Managed/Resource/ownership
behavior, diagnostic registry, schema, Semantic ID, source span, runtime,
support matrix, target, or Unicode 17.0.0 behavior. Registry parsing and
entries, comparison exemptions, equivalence, numeric/replay/cleanup/scheduling/
FFI/target predicates, conflict/expiry handling, protocol readers/migrations,
negative fixtures, and cross-target/property/fuzz claims remain deferred.
