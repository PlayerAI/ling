# DEC-0261: First-order handler runtime and bytecode / 一阶 Handler 运行时与字节码

> 状态：Accepted
> 提出日期：2026-08-24
> 决定日期：2026-08-24
> Owner role：effect-runtime-design
> 相关 RFC/缺口：RFC-0006 | RFC-0014 | RFC-0015 | RFC-0016 | RFC-0018 | RFC-0019 | RFC-0020 | DEC-0260 | GAP-EFFECT-HANDLER-001
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision closes the EFF-2104 execution authority for the operations that
the accepted source language can currently perform. It defines one reference
interpreter semantics and a verifier-gated `ling.bytecode/1.3` representation
without making handlers Stable or inventing Clock/Random source producers.

本决定关闭 EFF-2104 对当前已接受源码能够触发之 operation 的执行权威，规定唯一的参考
解释器语义和经 verifier 门控的 `ling.bytecode/1.3` 表示；它不把 Handler 提升为 Stable，
也不虚构 Clock/Random 源码 producer。

## Question

How must checked DEC-0260 handlers dispatch operations, capture and resume
continuations, interact with State, Faults, capabilities, and cancellation,
and lower to portable verified bytecode so that the interpreter and VM expose
the same deterministic behavior?

## Decision

1. **Checked-only entry.** Both backends consume one completed
   `ProgramSnapshot`. A `Handle` expression is executable only when its exact
   `ExpressionKey` has a matching `HandlerCore`; operation, result type, body
   identity, clause identity, and resume-use metadata must agree with checked
   HIR. Missing or inconsistent evidence is an internal checked-Core failure,
   never dynamic name lookup or fallback to unresolved AST interpretation.

2. **Current operation boundary.** `Console.write text` performs the canonical
   `Console.Write.write(Text)->Unit::Once` operation. The nearest dynamically
   active handler with that exact operation intercepts it; the host console is
   not called. Without a matching handler it retains the accepted host
   `Console.Write` behavior. `Clock.now` and `Random.next` clauses remain valid
   checked contracts but cannot dispatch until separately accepted source/Core
   producers exist. No operation is inferred from a label spelling.

3. **Deep lexical handlers.** Evaluation is strict and left-to-right. A
   handler evaluates its body with itself dynamically active. An intercepted
   operation captures the delimited continuation beginning immediately after
   that operation and ending at the handler body boundary. The selected clause
   executes with that handler removed and all outer handlers retained. A
   resumed continuation reinstalls the selected handler, so later matching
   operations in the continuation are handled deeply. An unmatched operation
   propagates to the next outer handler or its accepted host boundary.

4. **Clause and result behavior.** Operation inputs bind to clause parameters
   in source order. If declared, `resume` is a callable checked value of type
   `Output -> R`. Returning from the body without an operation yields the
   handler result directly. Returning from a clause without resuming yields the
   clause result. Calling `resume output` runs the captured continuation with
   `output` as the operation result; the resumed body result becomes the
   resume-call result, after which the remaining clause expression continues
   outside the selected handler. Every path returns the already checked `R`.

5. **Continuation lifetime and cardinality.** A continuation is VM/interpreter
   private, delimited, and valid only during the owning handler invocation. It
   is not serializable, comparable, printable, a Semantic ID, or a host handle.
   `Once` permits zero or one dynamic invocation and faults before a second
   invocation; `Many` permits repeated invocation subject to ordinary resource
   and cancellation limits. Each invocation starts from the same captured
   control point and lexical Cell identities. Invocations occur in source
   order; mutations and external effects already committed by an earlier
   invocation are visible and are never rolled back.

6. **State, Faults, and capabilities.** Handler dispatch does not catch or
   convert Runtime Faults. A Fault aborts the active clause, continuation, and
   handler stack; committed State mutation and host effects remain visible.
   `State<T>` remains unmasked. Capability preflight uses the unmasked checked
   closure, so a handled Console operation still requires declared and injected
   Console authority even though interception prevents that particular host
   call. Clause-body effects are outside the selected handler and may be caught
   only by an outer handler.

7. **Cancellation and limits.** Host cancellation is not a Ling Effect and
   cannot be handled or resumed. The VM checks the RFC-0020 token before every
   instruction, terminator, handler dispatch, and continuation restoration;
   cancellation wins before the next operation and preserves committed state.
   The reference interpreter exposes an explicit test/control entry point with
   equivalent monotonic cancellation checks at expression and continuation
   boundaries. Step, frame, heap, handler-depth, and continuation-frame limits
   fail before the charged action and use the existing structured Runtime Fault
   boundary. No cleanup syntax or rollback is implied.

8. **Reference interpreter.** `ling-eval` uses an explicit defunctionalized
   continuation machine. Continuation frames contain checked expression and
   binding identities plus lexical Cell references; they do not contain source
   paths, Rust closures, addresses, or debug text. The machine is the semantic
   oracle for successful values, ordered logical Console events, Fault facts,
   committed state, resume cardinality, and cancellation boundaries.

9. **Bytecode revision 1.3.** Format `(1,3)` is a backward-compatible minor
   extension of RFC-0014 through RFC-0016. Earlier readers reject it; the 1.3
   reader accepts exact 1.0–1.3 artifacts. It adds opcode `0x1c` `Handle` with
   operands `destination`, a zero-source-parameter closure-body function and
   ordered captures, followed by sorted unique clause records. Each clause
   record contains an operation tag (`1=Console.Write.write`, `2=Clock.now`,
   `3=Random.next`), a resume-present Boolean, two zero reserved bytes, one
   closure-body function index, and ordered captures. Existing capture encoding
   and hard vector limits apply. No continuation bytes or host handles appear
   in the artifact.

10. **Bytecode typing and effects.** The verifier requires the body closure to
    return `R`; each clause closure accepts the registered operation inputs and,
    when present, one final function value `Output -> R`, then returns `R`.
    Capture count/types, operation order, clause uniqueness, destination type,
    source-map coverage, function ownership, and resource bounds are exact.
    The residual Effect of `Handle` is the body closure Effect minus handled
    labels union clause Effects; Capability reachability separately includes
    the unmasked body and clauses. `ConsoleWrite` performs through the nearest
    active verified handler or the existing host instruction path.

11. **VM continuation ABI.** Executing `Handle` constructs body/clause closures
    from verified functions and captures, then calls the body under one bounded
    handler-stack entry. Interception snapshots only verified VM frames from
    the performing point through that entry. The resume argument is a
    VM-private continuation value satisfying the verified function type and is
    callable only through `CallClosure`; it is not constructible by bytecode.
    Restoring it pushes a bounded return-to-clause boundary, reinstalls the
    handler, supplies the operation result, and resumes at the next verified
    location. A body or clause return deterministically unwinds the matching
    boundary.

12. **Fault and differential contract.** Dynamic over-resume uses existing
    `L-RUNTIME-0001` with category `handler_resume_cardinality`, operation equal
    to the canonical operation name, the operation/resume source span, and the
    existing committed flag. Malformed handler encoding is `L-BYTECODE-*`;
    impossible verified state is `L-INTERNAL-0001`. Interpreter/VM comparison
    uses the RFC-0019 stable projection plus final value, ordered host events,
    resume count, and committed mutation observations. Physical paths,
    addresses, instruction counts, and allocation layout are never compared.

13. **Compatibility.** Non-handler interpreter results and bytecode 1.0–1.2
    bytes remain unchanged. Handler lowering selects only 1.3. Semantic Graph
    0.1 and Audit 0.1/0.2 shapes do not change. The feature remains
    Experimental v0.2 and preserves original UTF-8 spans and Unicode 17.0.0.

## Conformance plan

- Execute direct, sequenced, conditional, nested, callee-transitive, zero- and
  one-resume Console handlers in the interpreter; prove the nearest handler,
  deep resumption, outer clause-effect handling, no intercepted host write,
  and original BOM/CRLF/Unicode spans.
- Exercise a continuation through a higher-order call that attempts repeated
  invocation; require deterministic `handler_resume_cardinality` before the
  second Once restoration. Cover mutable Cell visibility and Fault propagation
  without rollback.
- Lower handler source only to bytecode 1.3; compare exact encode/decode,
  disassembly, canonical re-encoding, source maps, malformed operation/order/
  signature/capture/reserved-field cases, and rejection by older revisions.
- Verify handler/resume frame, step, heap, and cancellation boundaries before
  actions; ensure committed Console/State observations remain and no later
  operation executes.
- Add table-driven interpreter/VM differential fixtures for success, nested
  propagation, resume, State, Fault, cancellation, repeated construction, and
  path-independent bytes. Clock/Random execution fixtures remain deferred until
  accepted producers exist.

## Compatibility impact

- **Source/runtime:** activates execution only for already checked DEC-0260
  handlers and the existing `Console.write` producer; no new source spelling.
- **Bytecode:** plans public Experimental `ling.bytecode/1.3`; 1.0–1.2 readers,
  writers, bytes, and execution remain exact and reject the newer revision.
- **Diagnostics:** adds one `L-RUNTIME-0001` category/operation combination but
  no diagnostic code or schema field. Existing Fault categories are unchanged.
- **Semantic identity/data:** Handler-aware identities and Audit 0.2 already
  exist under DEC-0260; no new graph, Audit, CLI, LSP, package, or ABI field.
- **Determinism/Unicode:** source order, verified tables, and bounded frames
  determine behavior; Unicode stays 17.0.0 and spans remain original bytes.

## Unresolved alternatives

Clock/Random producers, `Never` operations, user-defined operations, shallow or
dynamic handlers, continuation serialization, cross-thread/task/actor resume,
rollback, cleanup/finally clauses, catching Fault/cancellation as Effects,
proof-carrying State masking, public debugger continuation inspection, Native/
Wasm lowering, migrations, and Stable compatibility remain separate work.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
