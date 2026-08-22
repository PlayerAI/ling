# EFF-2103-AST Authority Audit

## Outcome

`EFF-2103-AST` is complete as a bounded unresolved AST child under Accepted
DEC-0065. It consumes the DEC-0064 CST shape into explicit span-preserving
data, while HIR lowering rejects the value. The EFF-2103 parent remains
`BlockedSpec` for checked operation/effect lowering.

## Normative traceability

- RFC-0006 and DEC-0063 provide the first-order Effect/Handler and Core
  context; DEC-0064 fixes the parser/CST shape; DEC-0065 is the authority for
  this AST-only slice.
- The execution plan remains non-normative and does not authorize HIR or
  runtime semantics.
- Original UTF-8 spans, contextual Seed compatibility, deterministic source
  order, and the no-unchecked-AST execution boundary remain in force.

## Scope implemented

- `ExpressionKind::Handle` stores the body and source-order `HandlerClause`
  values.
- Each AST clause stores its `QualifiedName`, parameter `Pattern`s, optional
  contextual `resume` `Name`, body, and original span/source spelling.
- Malformed child shapes are rejected through existing structured AST lowering
  errors; no operation is resolved, reordered, or assigned a semantic ID.
- HIR lowering returns `UnsupportedHandler` before publishing an HIR value.

## Deferred

Operation namespace/signatures, binding identity, resume typing/cardinality,
return/recovery clauses, effect-row checking, checked Handler Core construction,
Audit Source/Semantic Graph fields, evaluator/VM execution, and Task/Actor
crossing remain parent or later-task scope.

