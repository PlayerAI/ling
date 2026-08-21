# EFF-2102 Authority Audit: Effect Inference and Constraints

## Outcome

`EFF-2102` is correctly recorded as `BlockedSpec`. The G2 execution plan asks
for operation/call row-constraint collection, row unification with occurs
checks, generalization/instantiation, minimal-conflict explanations, and
bilingual diagnostics. These algorithms depend on the unaccepted EFF-2101
model and RFC-C201/RFC-0006; their constraint language, handler subtraction,
State masking, and error meaning are not fixed.

No row-constraint solver, unification algorithm, generalized effect type,
handler subtraction, diagnostic code, Semantic ID change, or placeholder G2
API was added.

## Normative traceability

- The G2 execution package is non-normative; its three PR split and algorithm
  names do not authorize a public inference behavior.
- `docs/SEMANTICS.md` fixes the Seed closed, deduplicated Effect Row concept
  and conservative static checking, but does not accept row variables,
  constraint solving, handler elimination, or generalized effect schemes.
- Accepted DEC-0010 fixes Seed State/Capability visibility and call-graph
  authorization. It does not decide whether State can be masked or how a
  handler subtracts an effect from a function type.
- `GAP-EFFECT-STATE-MASKING-001` and `GAP-EFFECT-HANDLER-001` are Open and
  block EFF-2102; their candidate RFC-0006 is not Accepted. RFC-C201 is an
  execution-plan placeholder, not a repository document.
- The existing diagnostic registry requires stable bilingual `L-<DOMAIN>-<NUMBER>`
  allocations. No accepted code or schema defines row-conflict, occurs-check,
  unhandled-effect, or minimal-conflict payloads for v0.2.

## Current implementation evidence

- `ling-effects` collects direct Seed effects and propagates a deterministic
  fixed-point call graph over a closed `BTreeSet`-backed row. It has no row
  constraints, substitutions, levels, occurs check, generalization, or
  instantiation.
- Higher-order Seed call propagation and `State<T>`/`Console.Write`
  Capability checks are implemented, but there is no operation-signature
  constraint source for `Clock`, `Random`, `Task`, `ActorSend`, or user-defined
  effects.
- Current EffectError values cover missing and unknown Seed Capabilities only;
  no minimal unsatisfiable subset, related spans, canonical conflict order, or
  bilingual row explanation exists.
- No fixture covers equivalent constraint order, cyclic rows, polymorphic
  instantiation, value restriction, partial applications, handler subtraction,
  State masking, profile rejection, or deterministic diagnostic facts.

## Required authority before implementation

An Accepted RFC or decision must define, at minimum:

1. the constraint grammar and provenance for operations, calls, handlers,
   function types, row variables, levels, schemes, and module boundaries;
2. substitution/unification, row-tail normalization, occurs-check failure,
   generalization/instantiation, value restriction, partial application,
   recursion, and deterministic canonical ordering independent of traversal;
3. handler effect elimination/subtraction, resume typing, State masking and
   escape proofs, Capability closure, unhandled/profile failure, and the
   checked Typed Core representation consumed by evaluation;
4. conflict selection and explanation: minimal conflict-set definition,
   precedence, stable bilingual diagnostic codes/messages/facts/spans,
   recovery, related information, Semantic Graph/Audit Source fields, and
   schema/identity migration; and
5. executable positive/negative/property/migration and compiler-interpreter-
   VM differential fixtures for pure/Clock/Random rows, polymorphic `map`,
   nested handlers, occurs cycles, conflicting constraints, missing
   Capability, unhandled profile effects, State masking, Unicode/CRLF/BOM
   spans, randomized constraint order, canonical output, and deterministic
   minimal diagnostics.

Until these decisions are Accepted, a solver could infer a different function
type, hide a State effect without proof, emit unstable diagnostics, or allow
unchecked effect data to reach the evaluator.

## Evidence and compatibility

This audit was checked against `AGENTS.md`, `docs/SEMANTICS.md`,
`docs/LANGUAGE.md`, `docs/ERROR-CODES.md`, `docs/ROADMAP-1.0.md`, DEC-0010,
DEC-0017, `docs/ling_execution_plan/06-G2-V0.2-CONCURRENT.md`,
`docs/ling_execution_plan/13-IMPLEMENTATION-BACKLOG.md`,
`docs/governance/gap-register.toml`, `docs/governance/protocol-inventory.toml`,
and `crates/ling-effects`/`crates/ling-types`.

No compiler, interpreter, VM, bytecode, diagnostic, schema, Semantic ID,
source-span, runtime, or Unicode 17.0.0 behavior changed.

## Intentionally deferred

`EFF-2102` can begin only after EFF-2101's Effect/Handler authority and the
v0.1 exit are Accepted, with an independently reviewed RFC-0006 (or
replacement). The future solver must consume checked inputs, produce
canonical deterministic rows and stable bilingual conflicts, and prove
clean/differential equivalence before runtime integration.
