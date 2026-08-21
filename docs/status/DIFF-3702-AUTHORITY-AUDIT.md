# DIFF-3702 Authority Audit — Allowed-Difference Registry

Status: `BlockedSpec`

Date: 2026-08-21

## Outcome

DIFF-3702 proposes a machine-readable registry for the only differences that
may be ignored by the Interpreter/VM/Native conformance harness, such as
performance, unobservable addresses, unobservable GC timing, BestEffort
scheduling, and target-specific NaN payloads where the specification permits
them. The proposal is an execution-plan item, not an accepted semantic,
numeric, replay, Native, or compatibility registry.

No allowed-difference schema, entry, backend exemption, comparison predicate,
registry reader, versioning policy, or negative-test corpus is added. The
existing checked Interpreter/VM differential boundary remains unchanged, and
no Native difference is declared permissible until the governing semantics and
Native/FFI contracts are Accepted.

## Normative traceability

- `docs/ling_execution_plan/07-G3-V0.3-NATIVE.md:512-522` is non-normative.
  It names candidate differences but does not define registry fields,
  authority clauses, scope/predicate semantics, expiry, ownership, migration,
  or conflict handling. Its prohibition on scattered backend conditionals
  cannot itself authorize a registry.
- `docs/ROADMAP-1.0.md:70-78` requires canonical output to exclude host paths,
  HashMap order, Rust debug text, addresses, and thread scheduling details,
  while `:351-379` requires Native and differential evidence. These clauses
  do not decide which observations are semantically unobservable or permit
  target-specific numeric differences.
- Accepted RFC-0018/RFC-0019 govern the current experimental checked
  Interpreter–VM comparison projections. They do not define Native,
  cross-target, cleanup, replay, or allowed-difference behavior.
- `GAP-NATIVE-BACKEND-ABI-001` remains Open for Native ABI/layout,
  unwinding/Fault, thread/reentry, FFI, Target Primitive, and target tiers.
  `GAP-NUMERIC-CHECKED-FAULT-001` leaves fixed-width numeric failure and
  result rules unresolved; `GAP-DETERMINISTIC-REPLAY-001` leaves effect logs,
  event order, divergence, and equivalence unresolved; and
  `GAP-SEMANTIC-HASH-LIFECYCLE-001` leaves identity/schema versioning open.
- `PROTO-ABI`, `PROTO-EVIDENCE`, and a differential/allowed-difference
  protocol are not implemented in `docs/governance/protocol-inventory.toml`.
  No machine-readable public reader, migration tool, or fixture can be
  claimed for this task.
- RFC-N304, RFC-N305, RFC-N306, RFC-0004, RFC-0007, RFC-0010, and RFC-0011
  are not Accepted authorities in this repository; RFC-0001 remains Draft
  under DEC-0018.

## Current implementation evidence

- The workspace has no Native backend or three-engine harness and no
  allowed-difference registry. The existing Interpreter/VM differential tests
  compare the projections fixed by accepted VM RFCs and are not a basis for
  Native or target exceptions.
- No accepted rule defines floating-point tolerance, NaN payload equivalence,
  GC/Resource cleanup observability, BestEffort scheduling, replay divergence,
  FFI/target variation, or the boundary between an implementation defect and
  a permitted difference.
- No registry reader, schema, diagnostic allocation, Native dependency,
  toolchain, target, or public protocol implementation is required for this
  audit.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned registry schema with stable entry identity, authority clause,
   source/target engine scope, observable field, comparison predicate,
   rationale, status, owner, review/expiry, migration, and required positive
   and negative fixtures. Entries must be canonical and independently
   readable; absent or unknown entries must fail closed.
2. A semantic classification for each candidate: performance and host
   addresses/timing only when unobservable; GC/cleanup and BestEffort
   scheduling only under explicit lifecycle/determinism rules; and no
   suppression of values, events, Faults, capabilities, source spans, or
   Resource/Managed effects that are observable in the selected Profile.
3. Accepted numeric and target rules for precision, rounding, NaN payloads,
   signed zero, overflow/Fault, endianness, and declared tolerance, plus
   replay/effect-log and concurrency rules that define equivalence and
   divergence. FFI, ownership, ABI, and target contracts must bound all other
   differences.
4. Integration with DIFF-3701 projections and `PROTO-ABI`/`PROTO-EVIDENCE`,
   including provenance, program/target/profile/engine identity, version
   compatibility, migration, tamper/review evidence, and deterministic
   cross-process/offline behavior. Harness code must not branch on backend
   names outside the registry.
5. Stable bilingual diagnostics for unknown, expired, overlapping,
   contradictory, unauthorised, or out-of-scope entries, and release gates
   proving that every exception is specification-backed rather than a hidden
   snapshot or test waiver.

## Evidence and compatibility impact

The eventual implementation needs a canonical registry corpus; positive and
negative entries for performance, addresses, GC timing, BestEffort scheduling,
numeric/NaN tolerance, replay, cleanup, FFI, and target cases; unknown/expired/
overlap/conflict rejection; version migration and tamper tests; independent
reader and harness checks; cross-target/differential/property/fuzz evidence;
and deterministic offline provenance. It must preserve original UTF-8 byte
spans, stable Semantic IDs, bilingual `L-<DOMAIN>-<NUMBER>` diagnostics, and
Unicode 17.0.0 behavior without elevating host timing, addresses, allocation,
paths, or debug text to Ling semantics.

This audit changes no compiler, evaluator, bytecode, VM, scheduler, mailbox,
Actor protocol, dependency lock, memory category, Managed/Resource/ownership
behavior, diagnostic registry, schema, Semantic ID, source span, runtime,
support matrix, target, or Unicode behavior. It adds no registry entry,
comparison exemption, Native adapter, dependency, toolchain, diagnostic,
public protocol implementation, or placeholder API.

## Intentionally deferred

Allowed-difference schema and entries, numeric/NaN and replay equivalence,
cleanup/GC/scheduling/FFI/target predicates, conflict and expiry handling,
DIFF-3701 integration, protocol readers/migrations, negative fixtures,
cross-target/property/fuzz evidence, and all permitted backend-difference
claims remain deferred until the Native, numeric, replay, Semantic-ID,
ownership, FFI, `PROTO-ABI`, `PROTO-EVIDENCE`, and DIFF-3702 authorities are
Accepted.
