# EFF-2101 Authority Audit: Effect Core Model

## Outcome

`EFF-2101` is correctly recorded as `BlockedSpec`. The G2 execution plan asks
for `EffectId`, `EffectRow`, row variables, open/closed rows, operation
signatures, handler inputs and recovery types, explicit Capability separation,
canonical ordering, a Semantic Graph schema, and examples for `Clock`,
`Random`, `Console`, `State<T>`, `Task`, and `ActorSend<T>`. Its declared
dependency `RFC-C201` is not present as an accepted repository authority.
The bounded child `EFF-2101-SEED-ROW`, authorized by DEC-0060, adds only a
canonical in-process snapshot of the existing Seed closed row.

No open-row or handler model, new Effect/Capability label, `EffectId`, schema
field, diagnostic allocation, Semantic ID rule, or placeholder G2 crate/API was
added.

## Normative traceability

- The G2 execution package is non-normative; its RFC-C201 placeholder and
  acceptance table do not authorize v0.2 language behavior.
- `docs/SEMANTICS.md` describes Seed Effect Rows as unordered, deduplicated
  labels and marks user-defined handlers as future syntax. It does not accept
  row variables, open/closed-row inference, handler matching/elimination,
  resumption, or `Task`/`ActorSend` semantics.
- Accepted DEC-0010 fixes Seed `State<T>` visibility and Capability authority:
  local mutation contributes `State<T>`, external Capability requirements are
  checked before evaluation, and the evaluator cannot invent authorization.
  It does not define v0.2 row polymorphism or handlers.
- `GAP-EFFECT-STATE-MASKING-001` leaves State masking and escape proof open;
  `GAP-EFFECT-HANDLER-001` leaves polymorphism, matching, elimination,
  nesting, resumption, Capability interaction, and unhandled-effect failure
  open. Both identify candidate RFC-0006, not an Accepted RFC.
- The `Task` and actor labels in the proposed examples also depend on the
  unaccepted structured-task and actor lifecycle authorities. RFC-0001 remains
  Draft under DEC-0018 and cannot supply post-Seed authorization.

## Current implementation evidence

- `ling-effects` implements the Seed `EffectRow` as a deterministic set of
  `Console.Write` and parameterized `State` effects, propagates effects through
  the checked call graph, and checks the existing Capability closure.
- `EffectRow::seed_snapshot` exposes only deduplicated canonical names and
  pure-row state through `SeedEffectRowSnapshot`; it does not add a v0.2 row
  variable, handler, label, or wire/schema field.
- The current checker has no `EffectId`, row-variable constraints,
  open/closed-row distinction, user-defined operations, handler AST/Typed Core,
  resume multiplicity, effect discharge, or unhandled-handler failure model.
- `ling-semantic` can project the current Seed Effect/Capability facts, but no
  v0.2 Semantic Graph schema or canonical identity rule for new effect labels
  is accepted or inventoried.
- Existing tests cover Seed purity, `Console.Write`, `State<T>`, higher-order
  propagation, missing/unused Capability, and set ordering. No fixture covers
  row unification, generalization/instantiation, nested handlers, resumption,
  masking, `Clock`/`Random`/`Task`/`ActorSend`, or differential handler
  execution.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. Effect identity and canonical ordering, parameter normalization, equality,
   Semantic Graph/Audit Source representation, versioning, and migration;
2. open/closed rows, row variables, constraints, unification and occurs check,
   generalization/instantiation, higher-order and partial-application rules,
   and the boundary between static effects and runtime observations;
3. operation signatures, handler input/output and recovery types, scope,
   nesting/elimination, resume linearity or multiplicity, unhandled-effect
   behavior, and Typed Core lowering;
4. State masking/escape proofs, Capability requirements and injection, labels
   for Clock/Random/Console/Task/ActorSend, profile restrictions, diagnostics,
   and interaction with cancellation, replay, actors, and resource safety; and
5. executable positive/negative/migration/conformance and compiler-interpreter-
   VM differential fixtures for pure rows, Clock, handler discharge,
   polymorphic map, missing Capability, unhandled profile effects, nested and
   resumed handlers, State masking, Unicode/CRLF/BOM source spans, canonical
   ordering, and deterministic Semantic Graph bytes.

Until these decisions are Accepted, changing the existing `EffectRow` could
freeze an incompatible function-type or Capability contract, expose handler
control flow as language semantics, or make future Task/Actor behavior depend
on an accidental label representation.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ROADMAP-1.0.md`, DEC-0010, DEC-0018,
`docs/RFC-0001.md`, `docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and `crates/ling-effects`/`crates/ling-semantic`, including the bounded
`EFF-2101-SEED-ROW` report.

No compiler, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`EFF-2101-SEED-ROW` is complete under DEC-0060. The parent `EFF-2101` can
begin after v0.1 exit criteria, RFC-0006 (or an accepted replacement), and the
structured Task/Actor effect authorities are Accepted.
The future implementation must extend the Seed checker from an explicit
accepted model, preserve canonical deterministic rows, keep Capability
authorization separate, and provide checked Typed Core plus differential
evidence before any handler runtime is added.
