# DEC-0064: Handler source CST projection / Handler 源码 CST 投影

> 状态：Accepted  
> Status: Accepted  
> 提出日期：2026-08-22  
> 决定日期：2026-08-22  
> Owner role: effect-system-design  
> 相关 RFC/缺口：`RFC-0006`, `DEC-0063`, `GAP-EFFECT-HANDLER-001`  
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes the parser-only source projection for the bounded
`EFF-2103-SYNTAX` child. It fixes a lossless CST shape and contextual spelling
without authorizing an AST/HIR handler node, type/effect checking, evaluation,
bytecode, or a public protocol.

本决定仅授权有界 `EFF-2103-SYNTAX` 子任务的 parser-only 源码投影。它固定无损
CST 形状和上下文拼写，但不授权 AST/HIR Handler 节点、类型/Effect 检查、求值、
字节码或公共协议。

## Question

DEC-0063 defines the checked first-order Handler Core but deliberately leaves
the source grammar open. What smallest lossless parser boundary can record the
shape of a handler while keeping Seed source compatibility and preventing an
unchecked AST node from reaching later compiler stages?

DEC-0063 定义了一阶 checked Handler Core，但明确未规定源语法。需要一个最小的
无损 parser 边界记录 Handler 形状，同时保持 Seed 源码兼容，并阻止未检查的 AST
节点进入后续编译阶段。

## Decision

1. **Contextual spellings.** `handle`, `operation`, and `resume` are
   contextual identifier spellings in this projection; they are not globally
   reserved lexical keywords. Existing Seed identifiers remain identifiers
   unless the surrounding token shape matches this decision's handler form.

2. **Handler expression grammar.** The parser recognizes the following
   experimental CST form (brackets denote optional parts):

   ```text
   handler-expression := "handle" expression "with"
                         NEWLINE INDENT handler-clause+ DEDENT
   handler-clause    := "operation" qualified-name "("
                        [pattern ("," pattern)* ["," "resume"]]
                        ")" "->" body-expression
   body-expression   := expression
                       | NEWLINE INDENT sequence DEDENT
   ```

   The `with` delimiter is consumed at the handler-expression level. Nested
   `match ... with` expressions remain ordinary expression children. At least
   one operation clause is required. A clause body may use the existing Seed
   sequence and layout rules.

3. **Lossless CST shape.** A `HandleExpression` node has the body as its first
   child and one or more `HandlerClause` children in source order. A
   `HandlerClause` contains a `QualifiedName`, zero or more `Pattern` children,
   and its body expression. The contextual `resume` spelling remains in the
   clause token range and is not interpreted by the parser.

4. **Checked-only boundary.** The parser MUST preserve the original UTF-8
   token spans and MUST return bounded syntax errors for malformed forms. The
   AST lowerer MUST continue to reject this new CST kind until an additional
   Accepted decision defines binding identity, operation signatures, resume
   typing, return/recovery clauses, effect-row checking, and AST/HIR lowering.
   No evaluator, bytecode, VM, Semantic Graph field, or protocol may consume
   this CST projection.

5. **Compatibility and determinism.** Contextual recognition MUST be
   deterministic, independent of host paths, allocation order, map order, or
   debug formatting. CRLF, BOM, Unicode identifiers, and original byte spans
   follow the existing Seed lexer/layout rules.

## Conformance plan

- Parse one and multiple operation clauses, same-line and indented clause
  bodies, nested `match ... with`, Unicode qualified names, BOM, and CRLF
  sources; assert the exact node kinds, child ordering, and original spans.
- Reject missing `with`, missing indentation, empty clause lists, malformed
  operation names, unclosed parameter lists, missing arrows, and missing clause
  bodies with bounded parser errors.
- Verify Seed bindings named `handle`, `operation`, and `resume` remain valid
  outside the recognized handler token shape.
- Verify AST lowering rejects the experimental CST kind rather than publishing
  an unresolved AST/HIR or executing it.
- Repeat parsing with equivalent source bytes in independent processes and
  compare deterministic CST/token projections.

## Compatibility impact

- Seed source, existing token spellings, diagnostics, Semantic IDs, schemas,
  CLI, LSP, bytecode, VM, protocols, ABI, and Unicode 17.0.0 behavior: None.
- Adds an Experimental parser/CST node kind and no new diagnostic allocation.
  Existing AST lowering intentionally reports the existing unsupported-node
  error for this unimplemented semantic surface.
- Full source semantics, AST/HIR lowering, effect checking, Audit Source,
  interpreter/VM execution, and migration remain deferred.

## Unresolved alternatives

- The final operation signature, argument binding identity, explicit resume
  syntax and typing, return/recovery clause, nested propagation, mutable State,
  Fault/cancellation, and Task/Actor crossing require a later Accepted
  decision.
- A future Accepted authority may supersede this parser-only grammar with a
  different source form, but it must provide migration and corpus evidence.

## Supersession

- Supersedes: `None`
- Superseded by: `None`

