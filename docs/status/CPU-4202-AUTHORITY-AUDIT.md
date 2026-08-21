# CPU-4202 Authority Audit — Reference Trace

Status: BlockedSpec

Date: 2026-08-22

## Outcome

CPU-4202 proposes an optional test-mode trace for the future scalar Kernel
reference path. Its illustrative events contain a logical work item,
buffer reads/writes, an index, an operation, and a Fault. The plan explicitly
limits the trace to measurement and explanation rather than a stable
high-performance interface.

No trace producer, event schema, CLI flag, public protocol, snapshot corpus,
or runtime hook is added. Without an accepted scalar Kernel execution model,
even an internal trace would freeze work-item ordering, read/write identity,
numeric observation points, and Fault timing that the language has not
defined.

## Normative traceability

- docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:153-164 is a
  non-normative plan sketch. It gives example fields and a non-stability
  warning, but does not define event identity, ordering, granularity,
  sampling, redaction, schema version, corruption behavior, or source-map
  provenance.
- docs/ROADMAP-1.0.md:381-429 requires CPU-reference and differential
  evidence for G4 and keeps measurement output distinct from language
  semantics. It does not authorize a trace format or CLI surface.
- docs/SEMANTICS.md:1429-1480 and the v0.0.1 boundary at
  docs/SEMANTICS.md:1872-1928 describe future Kernel/device behavior and
  explicitly exclude Kernel execution from Seed. Existing VM control and
  replay foundations do not define a Kernel trace.
- GAP-KERNEL-DEVICE-001 is Open in docs/governance/gap-register.toml and
  blocks CPU-4202 through the unresolved Kernel execution, buffer,
  synchronization, determinism, Fault, and backend contracts.
- docs/governance/protocol-inventory.toml contains no Kernel reference-trace
  protocol. Any future external trace would need an explicit internal,
  Preview, or Stable classification and schema lifecycle.
- RFC-0014/0018/0019 cover scalar VM/bytecode verification and differential
  foundations only; RFC-0001 remains Draft under
  docs/decisions/0018-rfc-0001-lifecycle.md.

## Current implementation evidence

- The repository has no Kernel scalar backend, trace event type, trace
  serializer, test-mode switch, or reference corpus under crates or tests.
- No accepted rule fixes logical work-item identity, iteration and event
  ordering, buffer/view identity, read/write granularity, operation
  boundaries, reduction/atomic observations, Fault timing/provenance, or
  source/UTF-8 mapping in a Kernel execution.
- No accepted policy fixes trace size/resource limits, deterministic
  canonicalization, privacy/redaction, sensitive buffer contents, host
  addresses, driver information, or malformed-input behavior. The trace must
  not expose implementation details as Ling semantics.
- No diagnostic allocation, dependency, target/toolchain, public protocol,
  CLI command, or stable performance claim is required or changed by this
  audit.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. The CPU reference execution contract and event observation points,
   including work-item/index identity, buffer/view reads and writes,
   operations, reductions/atomics/barriers, numeric modes, and Fault
   ordering. The relation between trace order and program semantics must be
   explicit.
2. A trace schema classified as internal evidence or a versioned public
   protocol, with stable field meanings, source/UTF-8 spans and Semantic ID
   provenance, canonical ordering, versioning, migration, corruption
   handling, redaction, and bounded event/byte limits.
3. A verifier boundary that consumes checked Typed Core or a verified Kernel
   derivative, preserves original spans and identity, and cannot execute
   unchecked AST nodes merely to produce a trace.
4. Stable bilingual L-<DOMAIN>-<NUMBER> diagnostics for trace setup,
   unsupported execution, resource limits, malformed artifacts, missing
   provenance, and explicit Faults; non-stable debug details must remain
   outside the language error contract.
5. Positive and negative trace fixtures covering deterministic ordering,
   map/index/loop/buffer/reduction operations, invalid bounds, alias/race,
   numeric edges, Faults, cancellation, truncation/redaction, corruption,
   migration, Unicode/source-map positions, and offline reproducibility.

## Evidence and compatibility impact

The eventual trace must be an auditable consumer of verified execution, not
an alternate semantic interpreter. It should be clearly marked test/evidence
output, avoid host paths, addresses, allocation order, timing, driver logs,
or raw sensitive buffer data, and document that trace output is not a
performance or language compatibility guarantee. If the trace becomes
machine-consumed, it needs a protocol-inventory entry, schema fixtures,
reader/writer migration tests, and an explicit Preview or Stable status.

This audit changes no parser, Typed Core, evaluator, bytecode, VM, Native
backend, memory category, ownership behavior, effect or capability checker,
Device Buffer, scheduler, diagnostics, schema, Semantic IDs, source spans,
CLI, dependency lock, target/toolchain, support claim, or Unicode 17.0.0
behavior.

## Intentionally deferred

CPU-4202 implementation, trace events and serialization, test-mode CLI,
reference corpus, source-map mapping, redaction/limits, differential
harnesses, editor integration, and public protocol claims remain deferred
until CPU-4201 and GAP-KERNEL-DEVICE-001 are resolved by Accepted authority
and executable evidence exists.
