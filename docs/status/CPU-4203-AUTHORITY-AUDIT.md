# CPU-4203 Authority Audit — Kernel Corpus

Status: BlockedSpec

Date: 2026-08-22

## Outcome

CPU-4203 proposes a Kernel corpus containing vector addition, a small matrix
multiply, an image filter, reductions, an optional histogram/atomic case,
invalid bounds, alias conflicts, floating-point edge cases, and Unicode
source mapping.

The corpus cannot be added as executable or conformance evidence yet. No
Kernel fixtures, expected outputs, corpus manifest, differential oracle,
Fault snapshots, or test runner is added. Examples without an accepted
Kernel language and CPU-reference contract would turn unresolved choices into
de facto semantics.

## Normative traceability

- docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:165-180 is a
  non-normative corpus proposal. It lists topics but does not define source
  syntax, fixture metadata, artifact identity, expected result format,
  numeric tolerance, Fault classification, or version migration.
- docs/ROADMAP-1.0.md:381-429 requires a CPU reference before SIMD/device
  lowering and requires bounds, buffer, map/reduce, Fault, and differential
  evidence at the G4 exit. It does not authorize corpus inputs before the
  Kernel authority is Accepted.
- docs/SEMANTICS.md:1429-1480 sketches future Kernel and reduction behavior,
  while docs/SEMANTICS.md:1872-1928 explicitly reserves Kernel and excludes
  it from v0.0.1. Existing conformance fixtures cover the Seed language and
  cannot silently become Kernel fixtures.
- GAP-KERNEL-DEVICE-001 is Open in docs/governance/gap-register.toml. Its
  required evidence includes positive, negative, migration, CPU-reference,
  device-differential, bounds, and determinism corpus classes; its next
  action is an Accepted Kernel RFC and CPU reference corpus.
- CPU-4201 and CPU-4202 are now recorded as BlockedSpec because the scalar
  execution and trace contracts are absent. No Kernel corpus or protocol is
  registered in the governance inventories.

## Current implementation evidence

- The repository has no Kernel source fixtures, corpus manifest, CPU reference
  runner, trace snapshots, Device Buffer tests, reduction/atomic cases, or
  Kernel Unicode/source-map corpus under tests.
- No accepted rule fixes the fixture source representation, supported
  capabilities, shape/index/layout model, alias/race proof, reduction and
  floating-point behavior, invalid-input Faults, or CPU/device differential
  equivalence. A matrix multiplication or image filter would otherwise
  select unaccepted numeric and layout semantics.
- No accepted fixture schema defines stable program identity, source spans,
  expected output encoding, exact versus tolerance comparison, malformed
  input handling, resource limits, redaction, or migration. Host paths,
  driver versions, allocation order, and debug text must not become expected
  language output.
- No test command, dependency, target/toolchain, diagnostic allocation,
  public protocol, or support claim is required or changed by this audit.
  The authoritative source extension remains .ling; stale plan commands are
  not introduced.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. The Kernel source/Typed Core or verified-artifact contract, including
   supported types, control flow, effects/capabilities, shapes/indexes,
   buffers, ownership, reductions/atomics, Faults, numeric modes, and
   allowed targets.
2. A versioned corpus manifest and fixture schema for source bytes, program
   and Semantic IDs, specification clauses, profile/target, inputs,
   expected outputs, Faults, trace/evidence references, and migration
   metadata. Canonical ordering and corruption behavior must be explicit.
3. CPU-reference and differential rules that distinguish exact integer/
   structural equality from declared floating-point tolerances, reduction
   determinism, permitted backend differences, unsupported features, and
   fallback behavior.
4. A verifier boundary that consumes checked Typed Core or a verified
   derivative, preserves original UTF-8 byte spans and Semantic IDs, and
   never interprets unchecked AST nodes while running fixtures.
5. Stable bilingual L-<DOMAIN>-<NUMBER> diagnostics and structured facts for
   invalid bounds, alias/race conflicts, shape mismatch, numeric Faults,
   unsupported capability/target, malformed fixture, and resource limits.
6. Positive, negative, property, corruption, migration, Unicode/source-map,
   determinism, cancellation/resource, CPU-reference, and device-differential
   fixtures with offline reproducibility and no unverified claims.

## Evidence and compatibility impact

The eventual corpus must be executable only through verified artifacts and
must include deterministic manifest ordering, canonical inputs/outputs,
source-map and Unicode 17.0.0 coverage, explicit Fault provenance, and
documented exact/tolerance comparisons. Test fixtures are evidence, not a
replacement for a specification; they must not add syntax, runtime behavior,
diagnostics, schema fields, or backend guarantees that an Accepted authority
does not define. Any machine-consumed corpus format needs schema lifecycle,
reader/writer fixtures, and an explicit internal, Preview, or Stable status.

This audit changes no parser, Typed Core, evaluator, bytecode, VM, Native
backend, memory category, ownership behavior, effect or capability checker,
Device Buffer, scheduler, diagnostics, schema, Semantic IDs, source spans,
CLI, dependency lock, target/toolchain, support claim, or Unicode 17.0.0
behavior.

## Intentionally deferred

CPU-4203 implementation, Kernel source fixtures, corpus manifest, expected
outputs, Fault/trace snapshots, differential runner, Unicode/source-map
cases, editor integration, and public protocol claims remain deferred until
CPU-4201/4202 and GAP-KERNEL-DEVICE-001 are resolved by Accepted authority
and the required executable evidence exists.
