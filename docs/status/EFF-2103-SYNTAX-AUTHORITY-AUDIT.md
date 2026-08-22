# EFF-2103-SYNTAX Authority Audit

## Outcome

`EFF-2103-SYNTAX` is authorized and complete as a parser-only child of the
still-`BlockedSpec` EFF-2103 parent. Accepted DEC-0064 fixes the contextual
`handle`/`operation`/`resume` source shape and lossless CST projection. It does
not authorize AST/HIR lowering, type/effect checking, evaluation, bytecode, VM,
or a public protocol.

## Normative traceability

- RFC-0006 and DEC-0063 provide the first-order Effect/Handler and checked-Core
  context; DEC-0064 is the sole authority for this parser-only slice.
- The execution plan remains non-normative and is used only to identify the
  child boundary and required evidence.
- Seed source compatibility, original UTF-8 spans, deterministic token/CST
  ordering, and the checked-only compiler boundary remain in force.

## Scope implemented

- Contextual spellings are recognized without adding globally reserved lexer
  keywords, preserving Seed identifiers outside the handler token shape.
- `HandleExpression` contains a body followed by one or more `HandlerClause`
  nodes. Each clause contains a qualified operation name, parameter patterns,
  and a body; the optional contextual `resume` marker remains token evidence.
- Existing layout, nested `match ... with`, Unicode, BOM, and CRLF handling are
  reused. Malformed forms produce bounded parser errors.
- `ling-ast` rejects the experimental CST node rather than constructing an
  unresolved AST/HIR value.

## Deferred

Binding identity, operation signatures, resume typing/cardinality, return and
recovery clauses, effect-row checking, nested propagation, State/Fault/
cancellation behavior, Audit Source/Semantic Graph fields, interpreter/VM
execution, and Task/Actor crossing remain parent or later-task scope.

