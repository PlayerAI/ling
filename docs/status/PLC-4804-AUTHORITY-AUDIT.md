# PLC-4804 Authority Audit — Placement Explain Output

Status: BlockedSpec

Date: 2026-08-22

## Outcome

PLC-4804 proposes explain output for placement decisions, including candidate
devices, rejection reasons, chosen device, transfers, numeric mode, fallback,
cache hit/miss, and record/replay identity. Its plan heading uses the stale
`zero explain placement` name.

No explain command, output schema, CLI route, placement decision model, or
cache/replay report can be added yet. The accepted CLI is `ling`, not `zero`,
and no RFC-H405 or accepted Placement authority defines explain fields,
provenance, privacy, determinism, profiles, cache identity, or replay
compatibility. Introducing the stale command or an inferred schema would
create an unauthorized public protocol.

## Normative traceability

- `docs/ling_execution_plan/08-G4-V0.4-HETEROGENEOUS.md:480-493` is a
  non-normative implementation plan. It lists explain fields but does not
  define a command, machine-readable schema, field stability, decision
  provenance, rejection taxonomy, numeric/transfer semantics, cache identity,
  record/replay compatibility, privacy, or localization.
- `docs/ROADMAP-1.0.md:381-431` requires Placement decisions to be explicit,
  explainable, recordable, and replayable, but does not authorize a CLI or
  explain protocol. `docs/SEMANTICS.md` and `docs/LANGUAGE.md` do not accept
  Placement behavior for v0.0.1.
- `PLC-4801`, `PLC-4802`, and `PLC-4803` are `BlockedSpec`; Device IR,
  capability, runtime, cost, fallback, profile, cache, and replay authorities
  remain unresolved. `GAP-KERNEL-DEVICE-001` and
  `GAP-NATIVE-BACKEND-ABI-001` are Open.
- `AGENTS.md` and `docs/SEMANTICS.md` fix the public CLI as `ling` and source
  extension as `.ling`; stale `zero` references in the execution package must
  not enter implementation, fixtures, schemas, or editor integration.
- No `RFC-H405` or Accepted explain/placement protocol exists. The plan cannot
  create a public command or stable payload by itself.

## Current implementation evidence

- The repository has no Placement explain command, query/report API,
  machine-readable explain schema, decision provenance, rejection taxonomy,
  cache/replay identity, privacy filter, or explain fixtures under `crates` or
  `tests`.
- No accepted rule fixes candidate ordering, reason precedence, chosen-device
  identity, transfer accounting, numeric mode, fallback semantics, cache
  hit/miss meaning, record/replay versioning, or environment mismatch.
- No accepted policy distinguishes stable machine fields from diagnostic text,
  redacts paths/addresses/timestamps/driver/debug data, or defines how explain
  output is localized and mapped to registered diagnostics.
- No explain diagnostic allocation, public protocol, dependency, target, or
  CLI command is required or changed by this audit. The public CLI remains
  `ling`; no `zero` command is introduced.

## Required authority before implementation

Accepted decisions must establish, at minimum:

1. A versioned Placement decision and explain schema with canonical field
   ordering, stable versus diagnostic-only fields, provenance, source spans/
   Semantic IDs, rejection reasons, candidate/chosen identity, transfers,
   numeric mode, fallback, and profile information.
2. CLI and machine-readable transport contracts under the accepted `ling`
   command, including exit/diagnostic behavior, bilingual rendering, JSON
   stability, privacy/redaction, unknown-field handling, and migration.
3. Placement/Device IR, capability, runtime, cost, fallback, cache, and
   record/replay rules defining exactly which fields are evidence versus
   semantics and how Critical/Strict/Native profile differences behave.
4. Deterministic ordering, replay/cache identity, corruption rejection,
   environment mismatch, and no exposure of host paths, addresses,
   allocation order, timestamps, unstable driver text, or debug output as Ling
   identity.
5. Bilingual `L-<DOMAIN>-<NUMBER>` diagnostics and structured facts for
   unavailable candidates, rejection/conflict, fallback, transfer/numeric
   mismatch, cache/replay mismatch, cost/resource limits, and Faults.
6. Offline positive/negative, topology/capability, policy/cost, fallback,
   privacy, source-map/Unicode, migration, determinism, explain, replay, and
   differential fixtures.

## Evidence and compatibility impact

The eventual explain facility must report verified placement decisions and
must not become a second solver or a way to infer unsupported behavior. It
must use the accepted `ling` CLI and stable protocol lifecycle, keep stale
`zero` names out of public surfaces, and clearly distinguish estimates,
diagnostics, replay identity, and language semantics.

This audit changes no parser, resolver, type/effect checker, Typed Core,
evaluator, bytecode, VM, Native backend, memory category, ownership behavior,
Kernel or Device Buffer surface, diagnostics, schemas, Semantic IDs, source
spans, CLI, dependency lock, target/toolchain, support claim, or Unicode
17.0.0 behavior.

## Intentionally deferred

PLC-4804 implementation, `ling` explain command/protocol, output schema,
decision/rejection rendering, cache/replay evidence, privacy and migration,
diagnostics, editor integration, and public protocol claims remain deferred
until RFC-H405 (or an Accepted replacement), PLC-4801/4802/4803, and the
Kernel/Device IR, runtime, numeric, Native/backend, and support-matrix
authorities are Accepted. The stale `zero` name remains prohibited.
