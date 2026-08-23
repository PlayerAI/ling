# DEC-0260: Checked handler lowering / 已检查 Handler 降级

> 状态：Accepted
> 提出日期：2026-08-24
> 决定日期：2026-08-24
> Owner role：effect-system-design
> 相关 RFC/缺口：RFC-0006 | DEC-0062 | DEC-0063 | DEC-0064 | DEC-0065 | DEC-0066 | GAP-EFFECT-HANDLER-001 | GAP-EFFECT-STATE-MASKING-001
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision closes the checked-only EFF-2103 parent boundary. It connects the
accepted contextual source shape to deterministic resolution, type/effect
checking, `HandlerCore` publication, and the versioned Audit projection below.
It does not authorize handler execution, continuation storage, bytecode/VM
behavior, Task/Actor crossings, or a Handler-specific Semantic Graph extension.

本决定关闭仅限 checked 阶段的 EFF-2103 父任务边界，将已接受的 contextual 源码
形状连接到确定性的解析、类型/Effect 检查、`HandlerCore` 发布及下述版本化 Audit
投影；它不授权 Handler 执行、continuation 存储、bytecode/VM 行为、Task/Actor
跨越或 Handler 专用 Semantic Graph 扩展。

## Question

How does the compiler turn the DEC-0064 through DEC-0066 handler source/HIR
shape into the DEC-0063 checked Core without inventing runtime behavior or
changing Seed semantics outside an explicit Experimental handler expression?

## Decision

1. **Bounded operation registry.** This revision recognizes exactly these
   canonical source operations:

   | Source operation | Handled label | Inputs | Output | Resume mode |
   | --- | --- | --- | --- | --- |
   | `Console.Write.write` | `Console.Write` | `Text` | `Unit` | `Once` |
   | `Clock.now` | `Clock` | — | `Int` | `Once` |
   | `Random.next` | `Random` | `Int` | `Int` | `Many` |

   Names are NFC-normalized and case-sensitive. The registry is compiler-owned
   checked metadata, not an ordinary value namespace. Unknown operations and
   duplicate handled labels are rejected with `L-EFFECT-0005`; they create no
   ordinary reference or placeholder operation.

2. **Lexical bindings.** The handled expression is resolved in the enclosing
   scope. Each clause then receives a fresh child scope containing its
   source-order parameter-pattern bindings and optional resume binding. Clause
   bindings cannot escape, operation names are not value references, and
   duplicate/confusable binding rules remain the ordinary resolver rules.

3. **Types.** Clause parameter patterns are checked against the registered
   input types. If present, the resume binding has type `Output -> R`, where
   `Output` is the operation output and `R` is the handled expression's result
   type. Every clause body must have type `R`; the complete handler expression
   also has type `R`. Existing `L-TYPE-*` diagnostics report type mismatch.

4. **Resume use.** Resume use is the count of resolved references to that
   clause's resume binding within the clause body, including nested expression
   and local-function bodies. `Once` permits zero or one reference; `Many`
   permits any finite source reference count; `Never` permits no resume binder
   or reference. A violation is `L-EFFECT-0005`. This is a conservative
   first-order checked rule, not a runtime invocation-count promise.

5. **Effect rows.** The input row of a handler is the checked row of its body
   after nested handlers have been applied. A clause removes only its registered
   handled label and preserves every other label and open tail. Clause-body
   effects are outside that clause's interception and are unioned with the
   residual row. Call edges retain the lexical handled-label set so transitive
   callee effects are subtracted deterministically. `State<T>` remains visible
   because this registry declares no State operation.

6. **Capability separation.** Effect subtraction never grants or removes host
   authority. Capability closure is computed from unmasked reachable effects;
   therefore handling `Console.Write` still requires the existing
   `Console.Write` Capability even when the published residual row is pure.

7. **Checked Core publication.** Each successfully checked handler publishes
   one `HandlerCore` keyed by its module-local `ExpressionKey`. Body identities
   are the non-zero encoding `ExpressionId + 1`; return types use canonical
   checked type identities; clauses are canonicalized by DEC-0063 while source
   spans remain separate evidence. Failed resolution, typing, Effect checking,
   or Core construction publishes no `CheckedProgram`.

8. **Execution boundary.** The evaluator and every bytecode/VM lowering path
   continue to reject `Handle`; EFF-2104 alone may authorize execution and
   continuation behavior. Since Task and Actor checked constructs do not yet
   exist, a handler cannot cross those boundaries. Fault, cancellation,
   continuation storage, and mutable-State execution remain undefined here.

9. **Graph and Audit projection.** The existing `ling.semantic/0.1` graph
   publishes Handler, clause-body, parameter, resume-binding, and reference
   identities through its existing expression/binding/reference node kinds;
   no Handler-specific Semantic Graph field is added. Canonical Audit Source
   revision `ling.audit/0.2` adds one `handler` block per checked Core with the
   graph expression identity, expression/body ordinals, original UTF-8 byte
   span, return type, input row, explicitly eliminated labels, residual row,
   and canonical operation/label/body/resume mode/use clauses. The isolated
   reader validates those relationships and cannot produce executable Core.
   Models without handlers continue to render the byte-compatible
   `ling.audit/0.1`; the reader accepts both exact versions, and 0.1 rejects
   Handler core fields. The protocol inventory registers both markers and the
   compatibility rule.

10. **Compatibility.** The contextual keywords remain ordinary identifiers
    outside DEC-0064's grammar position. Non-handler Seed programs and their
    accepted facts are unchanged. The feature remains Experimental v0.2 and
    does not alter Unicode 17.0.0, original UTF-8 spans, runtime, ABI, or Stable
    compatibility.

## Conformance plan

- Compile single and nested handlers from source through resolution, typing,
  Effect checking, and `HandlerCore`; compare result type, input/residual rows,
  clause order, binding targets, spans, and canonical bytes.
- Cover zero/one/multiple resume references, missing/extra parameters,
  duplicate labels, unknown operations, type mismatch, shadowing, Unicode,
  BOM, and CRLF with stable bilingual diagnostics.
- Prove transitive and nested Effect subtraction, clause-body propagation,
  visible `State<T>`, and the requirement that handled `Console.Write` still
  has declared Capability authority.
- Prove failed checking publishes no checked Core, Graph, or Audit output;
  prove Handler Audit 0.2 canonical render/parse equality and that evaluator
  and all bytecode/VM lowerers retain structured rejection.
- Run focused model/conformance tests and all locked offline workspace,
  Clippy, CI, governance, support, status, RC0, traceability, formatting,
  checksum, and deterministic-diff gates.

## Compatibility impact

- **Source/compiler:** adds the already parsed Experimental handler expression
  to successful resolution, typing, Effect checking, and checked-Core output
  for the exact registry above.
- **Diagnostics:** allocates `L-EFFECT-0005` for invalid operation/arity/resume
  contracts; existing diagnostic meanings and original byte spans are retained.
- **Effects/Capabilities:** handler residual rows become observable through the
  existing in-process checked API; host Capability requirements remain based on
  unmasked uses.
- **Runtime/bytecode/VM/ABI:** unchanged and explicitly rejecting.
- **Schema/Semantic ID/CLI/LSP/data:** adds the accepted `ling.audit/0.2`
  Handler block and Handler-aware Semantic ID input while preserving
  non-handler `ling.audit/0.1`, Semantic Graph 0.1 shapes, IDs, and bytes.
- **Determinism/Unicode:** canonical registry/order/count rules depend only on
  checked source facts and Unicode 17.0.0 normalization, never paths, map order,
  allocation, timing, threads, or debug output.

## Unresolved alternatives

User-declared Effects and operations, polymorphic operation signatures,
`Never` source operations, proof-carrying State masking, dynamic handlers,
continuation capture/storage, runtime invocation cardinality, Fault and
cancellation interaction, Task/Actor crossings, interpreter/VM execution,
package-aware Handler Audit, migration tooling, and Stable compatibility remain
deferred to separately accepted authorities.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
