# EFF-2103-HIR Authority Audit

## Outcome

`EFF-2103-HIR` is complete as a bounded unresolved HIR projection under
Accepted DEC-0066. It preserves the DEC-0065 AST shape through HIR and makes
resolution the first rejecting gate. The EFF-2103 parent remains `BlockedSpec`
for checked operation signatures, resume typing, Effect checking, and runtime
semantics.

## Normative traceability

- RFC-0006 and DEC-0063 provide the first-order Effect/Handler and checked-Core
  context; DEC-0064 fixes the contextual CST; DEC-0065 fixes the unresolved AST;
  DEC-0066 is the authority for this HIR/resolution slice.
- The execution plan is non-normative and cannot authorize handler semantics by
  itself.
- Original UTF-8 spans, deterministic source order, contextual identifiers,
  and the no-unchecked-handler execution boundary remain in force.

## Scope implemented

- `ling-hir` now stores unresolved `ExpressionKind::Handle` values with
  source-order `HandlerClause` data, lowered operation names, parameter
  patterns, optional resume names, and clause bodies.
- HIR allocates ordinary expression, pattern, and binding IDs but creates no
  operation or resume references and performs no operation lookup, scope
  resolution, resume typing, or Effect-row construction.
- `ling-resolve` rejects every unresolved handler with structured
  `ResolveErrorKind::UnsupportedHandler` and bilingual `L-EFFECT-0004`; no
  `ResolvedProgram` or handler references are published.
- Type, Effect, Semantic Graph, evaluator, CLI rewriting, and bytecode paths
  contain explicit non-interpreting/rejection branches so direct invalid HIR
  cannot silently become executable behavior.

## Deferred

Operation namespaces/signatures, binding identity, resume cardinality and
typing, return/recovery clauses, nested propagation, State/Fault/cancellation,
Task/Actor crossing, checked Handler Core publication, Audit Source/Semantic
Graph fields, interpreter/VM continuation behavior, migration, and public
protocols remain parent or later-task scope.

