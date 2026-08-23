# EFF-2103 Authority Audit: Handler Typed Core

## Outcome

`EFF-2103` has complete Accepted authority for its checked-only parent slice.
RFC-0006 defines the first-order Effect/Handler model, DEC-0062 defines row
solving, DEC-0063 through DEC-0066 define the Core/CST/AST/HIR children, and
DEC-0260 defines the missing operation registry, lexical binding, type/resume
checking, residual-row computation, Capability separation, Core publication,
Semantic Graph identity traversal, Audit Source 0.2, and execution gates.

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
- DEC-0065 closes the span-preserving unresolved AST data shape for
  `EFF-2103-AST`; DEC-0066 extends it through unresolved HIR.
- DEC-0066 closes only unresolved HIR data and the `L-EFFECT-0004` resolver
  rejection used before parent authority existed; DEC-0260 replaces that gate
  for its exact fixed registry without changing the historical diagnostic.
- DEC-0260 fixes three canonical built-in operations, lexical parameter/resume
  bindings, `Output -> R` resume types, clause result unification, syntactic
  resume-use bounds, nested/transitive Effect subtraction, unmasked Capability
  closure, deterministic Core/Graph/Audit publication, and `L-EFFECT-0005`
  failures.
- Task/Actor crossing, runtime continuations, Fault/cancellation behavior,
  interpreter/VM ABI, package-aware Handler Audit, and public protocol migration
  remain outside these authorities.

## Current implementation evidence

- The child `EFF-2103-CORE` implements `HandlerCore`, canonical clauses,
  residual-row computation, resume-use validation, source-span evidence, and
  `L-EFFECT-0003` closed-boundary diagnostics in `ling-effects`.
- The `EFF-2103-SYNTAX` child parses contextual `handle` expressions and
  operation clauses into lossless `HandleExpression`/`HandlerClause` CST nodes.
- The `EFF-2103-AST` child lowers those nodes into explicit unresolved AST
  values, and `EFF-2103-HIR` preserves them through HIR before structured
  resolver rejection.
- `ling-resolve` validates the DEC-0260 registry, publishes lexical clause and
  resume bindings, and enforces arity, duplicate-label, and resume-use rules.
- `ling-types` checks parameter, resume, handler-result, and clause-result types
  while preserving ordinary trait-obligation traversal.
- `ling-effects` propagates lexical handled-label masks across call edges,
  computes unmasked Capability closure, and publishes one `HandlerCore` per
  successfully checked handler expression.
- `ling-semantic` includes Handler children and resume bindings in existing
  expression/binding/reference identities and builds a path-free Audit model;
  `ling-format` renders and independently parses canonical `ling.audit/0.2`
  Handler blocks with input/eliminated/residual rows and original byte spans.
- CLI `check`, `semantic`, and `audit` can publish checked Handler evidence;
  evaluator and all bytecode/VM lowerers remain rejecting under EFF-2104.
- Non-handler Seed checking and runtime behavior remain unchanged.

## Parent closure boundary

DEC-0260 closes only source-to-checked-Core representation. It deliberately
does not reinterpret the parent as runtime work: continuation capture/storage,
Fault/cancellation and mutable-State execution, Task/Actor crossing,
interpreter/VM behavior, and differential execution remain EFF-2104 or later.
The existing Semantic Graph node kinds carry Handler identity without a new
Handler-specific field. Audit Source 0.2 is the bounded public projection;
package coordinates, executable reconstruction, migration tooling, and Stable
compatibility remain outside this task.

## Compatibility and deferred work

The exact Experimental handler syntax now reaches checked Core, Graph identity,
and Audit Source 0.2. It allocates `L-EFFECT-0005`; the provisional
`L-EFFECT-0006` schema-gate allocation is retired without an emitter. Existing
diagnostic meanings and non-handler Seed acceptance, Semantic IDs, Semantic Graph
and Audit 0.1 bytes, runtime, bytecode, VM, ABI, and Unicode 17.0.0 remain
unchanged. EFF-2104 execution and package-aware/Stable projection work remain
deferred.
