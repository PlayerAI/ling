# DEC-0265: Checked Task state-machine lowering / Checked Task 状态机 Lowering

> 状态：Accepted<br>
> 提出日期：2026-08-25<br>
> 决定日期：2026-08-25<br>
> Owner role：concurrency-design<br>
> 相关 RFC/缺口：DEC-0092 | DEC-0264 | GAP-STRUCTURED-TASK-001 | TASK-2202<br>
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision closes only the checked state-machine lowering boundary of
TASK-2202. It does not authorize Task execution, lifecycle behavior,
scheduling, bytecode instructions, detach, deadlines, or a production runtime.

本决定仅关闭 TASK-2202 的 checked 状态机 lowering 边界，不授权 Task 执行、
生命周期行为、调度、bytecode 指令、detach、deadline 或生产 runtime。

## Question

What deterministic, versioned, non-executable state-machine representation may
be lowered from the Accepted DEC-0264 Checked Task Core so TASK-2203 can later
define runtime lifecycle semantics without re-reading unchecked AST/HIR or
exposing a premature bytecode ABI?

## Decision

1. **Checked input and ownership.** Effect checking atomically lowers every
   Accepted `CheckedTaskCore` into one immutable `CheckedTaskMachine` owned by
   `CheckedProgram`. The pass may inspect the corresponding checked HIR body
   through `TypedProgram` only to recover evaluation-order control flow; every
   suspension, scope, spawn, live binding, type, cancellation identity, and
   cleanup identity must match the Checked Task Core. Missing, duplicate, or
   unmatched checked evidence rejects publication. AST, unresolved HIR, source
   text, and debug output are never lowering inputs.

2. **Version and scope.** The internal format identifier is exactly
   `ling.task-machine/0.1`. It is Experimental, publish-disabled, and not a
   public wire, artifact, bytecode, replay, or Stable protocol. One machine is
   emitted per Task declaration. Task declarations without suspension still
   emit entry, cleanup, and terminal control states.

3. **States.** State identities are deterministic source-order ordinals within
   the canonical Task definition owner. A machine contains exactly one
   `Entry`, one `Suspend` state per Checked Task suspension, three reasoned
   cleanup states (`ReturnCleanup`, `CancelCleanup`, `FaultCleanup`), and three
   terminal states (`Completed`, `Cancelled`, `Faulted`). A `Suspend` state
   carries its `SuspensionId`, owning `ScopeId`, awaited `TaskId`, checked
   continuation `ExpressionKey`, original UTF-8 span, and exact sorted typed
   frame slots. Other states carry no frame slots.

4. **Typed frames.** Each frame slot is the exact DEC-0264 live binding key and
   checked `TypeId` at that suspension. Slot order is `(module ID, binding ID)`.
   Lowering neither recomputes liveness nor captures mutable bindings, Handler
   continuations, unresolved obligations, or other Task handles. Frame layout,
   allocation, addresses, ownership representation, and ABI offsets remain
   unspecified until an executable backend decision.

5. **Normal control flow.** Checked HIR evaluation order determines edges among
   `Entry`, `Suspend`, and `ReturnCleanup`. `Continue` reaches a suspension or
   successful cleanup without resumption; `Resume` leaves a suspension after
   its awaited result becomes available. Sequential expressions preserve
   order. `if` and `match` union branch successors and converge only where the
   checked source converges. Nested lexical scopes remain in the same Task
   machine and retain their Checked scope identities. Duplicate edges are
   canonicalized. A reachable source path that has no final `return`, or
   checked suspension evidence that is unreachable or used more than once on
   one path, is rejected as an internal lowering disagreement.

6. **Exit skeleton.** Every active state (`Entry` or `Suspend`) has a `Cancel`
   edge to `CancelCleanup` and a `Fault` edge to `FaultCleanup`. Normal return
   reaches `ReturnCleanup`. Each reasoned cleanup state has exactly one
   `Cleanup` edge to its matching terminal state. These edges preserve the
   Checked Task cleanup/cancellation identities as structural obligations only;
   they do not define propagation, cleanup code, Fault aggregation, precedence,
   timing, or execution.

7. **Validation.** Construction rejects an unknown version, invalid or
   duplicate state/edge identity, missing entry/cleanup/terminal state,
   mismatched state role, unknown endpoint, duplicate edge, invalid terminal
   outgoing edge, incomplete exit skeleton, mismatched suspension/frame/core
   identity, unreachable state, or a normal path that cannot reach
   `ReturnCleanup`. Cycles are accepted by the data model only when an Accepted
   future checked source form supplies the back edge; v0.2 currently has no
   source loop form, so compiler lowering emits no back edge.

8. **Identity, bytes, and source maps.** Canonical bytes include the exact
   format identifier, Task definition identity, state roles and identities,
   typed frame identities/types, edge kinds/endpoints, and structural
   cancellation/cleanup identities in canonical order. They exclude source
   paths, spans, source IDs, allocation order, hash-map order, host addresses,
   and debug strings. Source maps retain original UTF-8 spans as non-canonical
   evidence. Equivalent reconstruction and physical-path changes produce
   identical bytes.

9. **Publication boundary.** `CheckedProgram::task_machine` and
   `task_machines` expose only validated immutable machines. Existing Semantic
   Graph `x-ling-task/0.1` and Audit Source 0.3 remain unchanged in TASK-2202;
   no machine protocol is published. File/project run and test, REPL,
   interpreter, project artifact, bytecode 1.0–1.4, and VM paths continue to
   reject checked Task programs with `L-TASK-0004` before evaluation or output.

10. **Later authority.** TASK-2203 must separately define executable scope
    creation/closure, child registration/join, cancellation propagation,
    cleanup execution and precedence, child Fault aggregation, resource limits,
    and interpreter semantics. TASK-2204 through TASK-2206 retain scheduler,
    production runtime, conformance, and stress authority. Any Task bytecode or
    VM representation requires a separate Accepted revision and verifier plan.

## Conformance plan

- Lower zero, one, repeated, nested-scope, conditional, and match-branch
  suspension programs; freeze exact state roles, typed frames, transitions,
  source maps, and canonical bytes.
- Prove mutually exclusive branch suspensions are not sequenced, convergent
  paths share only their actual successor, and every reachable normal path
  reaches `ReturnCleanup` then `Completed`.
- Verify each active state has explicit cancellation and Fault exits and every
  return/cancel/Fault cleanup state has exactly one reason-preserving terminal
  edge.
- Reject missing/duplicate/unmatched suspension evidence, incorrect live types,
  malformed identities/roles/endpoints, incomplete exits, unreachable states,
  invalid terminal edges, and normal paths without return cleanup.
- Prove insertion-order, physical-path, source-ID, span, BOM/CRLF, and Chinese
  identifier independence while retaining exact original UTF-8 source spans.
- Keep `L-TASK-0004` rejection evidence for file/project run/test/build, REPL,
  interpreter, bytecode 1.0–1.4, and VM, with no output or artifact publication.
- Exercise a synthetic validated back-edge model as the loop boundary; defer
  source-loop lowering until a loop form and its ownership semantics are
  Accepted.

## Compatibility impact

- Adds an internal Experimental checked Task machine and accessors to
  `CheckedProgram`; ordinary checked programs and existing Task Core,
  Semantic Graph, and Audit bytes remain unchanged.
- Adds no source syntax, public diagnostic meaning, CLI success path, artifact,
  schema, public protocol, bytecode instruction/version, runtime value,
  scheduler order, ABI, package behavior, Native/Wasm behavior, or Stable
  compatibility promise.
- Machine canonical bytes are a new internal version and do not reinterpret
  DEC-0092 `ling.task-state-machine/0` identity-model bytes.
- Original UTF-8 spans and Unicode 17.0.0 remain authoritative; paths and host
  presentation remain non-semantic.

## Unresolved alternatives

- Direct AST/HIR execution, source-order-only sequencing of mutually exclusive
  branches, implicit cleanup, untyped frames, allocation/layout commitments,
  and reusing Seed call frames as Task continuations are rejected.
- Runtime propagation/precedence, cleanup bodies, child Fault aggregation,
  detach, deadlines, test and production schedulers, worker pools, resource
  limits, Task bytecode/VM/native ABI, Replay, Actor crossing, source loops,
  migration, and Stable compatibility remain TASK-2203 through TASK-2206 or
  later Accepted work.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
