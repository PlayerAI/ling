# EFF-2103 Authority Audit: Handler Typed Core

## Outcome

`EFF-2103` is correctly recorded as `BlockedSpec`. The G2 plan proposes an
explicit `handle`/`with operation` Core form, requires RFC-defined linear or
multiple `resume` behavior, explicit Task/Actor boundary rules, rejection of
runtime fallback for unhandled effects, and Audit Source expansion of handled
and residual rows. No accepted RFC fixes that syntax, Typed Core shape,
resumption, scope, or source projection.

No handler syntax, AST/HIR/Typed Core node, effect-discharge rule, resume
checker, Audit Source field, diagnostic allocation, or placeholder G2 API was
added.

## Normative traceability

- The G2 execution package is non-normative; its pseudo-syntax and lowering
  requirements do not authorize a new source construct or Core node.
- `docs/SEMANTICS.md` describes handlers as future typed Effect behavior and
  does not accept handler matching, nesting, resumption, or residual-row
  semantics. The current v0.0.1 Seed grammar excludes these constructs.
- Accepted DEC-0010 defines State visibility and Capability authorization, but
  not effect handling, discharge, handler scope, or resume ownership.
- `GAP-EFFECT-HANDLER-001` keeps matching, elimination, nesting, resumption,
  Capability interaction, and unhandled-effect failure open; its candidate
  RFC-0006 is not Accepted. `GAP-EFFECT-STATE-MASKING-001` also remains open.
- Task and Actor crossing is governed by separate unaccepted RFC-C202/C203/C204
  placeholders and structured lifecycle gaps. No handler may cross those
  boundaries by inference from the execution plan.

## Current implementation evidence

- `ling-syntax`, `ling-ast`, `ling-hir`, `ling-types`, and `ling-effects` have
  no handler grammar, HIR/Typed Core node, operation clause, resume binding,
  handler scope, or effect-discharge representation.
- `ling-effects` only propagates Seed closed rows and validates module
  Capabilities. It cannot distinguish handled from residual effects or reject
  an unhandled effect at a profile boundary.
- `ling-format` and Semantic Graph projections have no accepted handler
  rendering, residual-row field, source identity, or schema version.
- No fixture covers single/nested handlers, operation matching, linear versus
  multiple resume, handler propagation, Fault/mutable-State interaction,
  cancellation, Task/Actor crossing, source spans, or interpreter/VM
  differential behavior.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. source grammar, AST/HIR/Typed Core node shape, operation signatures,
   handler scope and environment, binding identity, source spans, and lowering
   invariants;
2. effect matching/elimination, residual rows, nesting and propagation,
   operation parameters, return/recovery types, resume typing, linear versus
   multiple invocation, recursion, and unhandled-effect rejection;
3. handler interaction with State masking/escape, Capability injection,
   mutable places, Faults, cancellation, Task scopes, Actor turns, replay,
   resource cleanup, and profile restrictions;
4. checked-only evaluation boundary, interpreter/VM ABI and differential
   semantics, Audit Source/ Semantic Graph representation, deterministic
   ordering, diagnostics, protocol/version migration, and no runtime fallback;
5. executable positive/negative/migration/differential fixtures for single and
   nested handlers, operation propagation, resume cardinality, residual rows,
   missing/unhandled effects, Fault and mutable State, cancellation, Task/Actor
   boundaries, Unicode/CRLF/BOM spans, and canonical Audit output.

Until these decisions are Accepted, a Core node could allow unsound resume
aliasing, hide residual effects, cross a Task/Actor boundary incorrectly, or
let the evaluator execute an unvalidated handler.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0010, DEC-0018,
`docs/RFC-0001.md`, `docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and the current syntax, AST, HIR, type, effect, format, semantic, evaluator,
and bytecode crates.

No compiler, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`EFF-2103` can begin only after EFF-2101/EFF-2102 and an Accepted RFC-0006 (or
replacement) plus explicit Task/Actor boundary decisions. The future
implementation must lower only accepted handler syntax into checked Typed
Core, preserve residual effects and source identity, reject unhandled effects,
and prove interpreter/VM equivalence before any runtime handler is exposed.
