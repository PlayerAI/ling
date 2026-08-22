# EFF-2103 Authority Audit: Handler Typed Core

## Outcome

`EFF-2103` remains `BlockedSpec` for the complete `handle`/`with operation`
AST/HIR lowering and checking contract. RFC-0006 is Accepted for the
first-order Effect/Handler model, DEC-0062 is Accepted for row solving,
DEC-0063 authorizes the bounded `EFF-2103-CORE` checked projection, and
DEC-0064 authorizes only the parser/CST `EFF-2103-SYNTAX` child. Neither child
makes the parent complete.

## Normative traceability

- The G2 execution package is non-normative; its pseudo-syntax and lowering
  order do not authorize a public construct by themselves.
- `docs/SEMANTICS.md` and `docs/LANGUAGE.md` describe handlers as future
  behavior and do not fix source grammar, AST/HIR nodes, or resume syntax.
- RFC-0006 §§4, 6, 7, and 9 fix canonical rows, operation contracts, lexical
  residuals, resume cardinality, Capability separation, and path-free
  projections, while explicitly assigning source syntax and Checked-Core
  lowering to EFF-2103.
- DEC-0062 fixes deterministic row constraints and bilingual conflict facts.
- DEC-0063 closes only the checked first-order Core container, clause body
  identities, resume-use invariant, and explicit closed-boundary residual
  diagnostic for `EFF-2103-CORE`.
- DEC-0064 closes only the contextual, lossless parser/CST shape for
  `EFF-2103-SYNTAX`; it explicitly rejects AST/HIR publication and execution.
- Task/Actor crossing, runtime continuations, Fault/cancellation behavior,
  Audit Source schema, interpreter/VM ABI, and public protocol migration remain
  outside these authorities.

## Current implementation evidence

- The child `EFF-2103-CORE` implements `HandlerCore`, canonical clauses,
  residual-row computation, resume-use validation, source-span evidence, and
  `L-EFFECT-0003` closed-boundary diagnostics in `ling-effects`.
- The `EFF-2103-SYNTAX` child now parses a contextual `handle` expression and
  operation clauses into lossless `HandleExpression`/`HandlerClause` CST nodes;
  the AST lowerer rejects that experimental node before publication.
- No AST/HIR handler node, TypedProgram integration, evaluator, bytecode
  instruction, VM handler stack, or public schema has been added.
- The existing Seed checker and runtime remain unchanged.

## Required authority before parent completion

An additional Accepted decision must still define, at minimum:

1. exact AST/HIR lowering and checked publication for the parser shape fixed by
   DEC-0064;
2. binding identity, lexical scope, operation arguments, return/recovery types,
   resume syntax, source spans, and checked-only publication invariants;
3. nested propagation, unhandled-effect policy, mutable State/Fault/cancellation
   interaction, and explicit Task/Actor boundary behavior; and
4. Audit Source/Semantic Graph fields, interpreter/VM differential contracts,
   migration, and executable positive/negative fixtures.

Until those decisions are Accepted, the parent must not publish source syntax
or let an evaluator execute a handler node. The child remains a reusable Core
projection only.

## Compatibility and deferred work

No Seed source acceptance, existing diagnostic meaning, Semantic ID, schema,
CLI, LSP, runtime, bytecode, VM, protocol, or Unicode 17.0.0 behavior changes
under the current child. Full parent completion and EFF-2104/2105 execution
remain deferred.
