# DEC-0264: Structured Task frontend and Checked Core / Structured Task 前端与 Checked Core

> 状态：Proposed<br>
> 提出日期：2026-08-24<br>
> 决定日期：Pending<br>
> Owner role：concurrency-design<br>
> 相关 RFC/缺口：DEC-0091 | DEC-0260 | GAP-STRUCTURED-TASK-001 | TASK-2201<br>
> 生命周期记录：`docs/governance/lifecycle.toml`

This proposal closes only the source-to-checked boundary of TASK-2201. It does
not authorize Task state-machine lowering, execution, scheduling, detach,
deadlines, or a production runtime.

本提案仅关闭 TASK-2201 从源码到 checked 表示的边界，不授权 Task 状态机 lowering、
执行、调度、detach、deadline 或生产 runtime。

## Question

What is the smallest source grammar, static ownership rule, latent Effect rule,
and immutable Checked Core projection that can represent lexical Structured
Tasks without publishing runtime behavior before TASK-2202 through TASK-2206?

## Decision

1. **Contextual syntax.** `task`, `scope`, `spawn`, `await`, and `return` are
   contextual words, not globally reserved identifiers. `let!` is the adjacent
   token sequence `let` `!` in a Task scope. A Task declaration has the form
   `task name parameters = scope-block`. A Task scope is an indented block of
   ordinary local `let` declarations and expressions plus Task forms. Every
   Task declaration has one outer `scope`; nested `scope` blocks are allowed.
   `return expression` must be the final reachable element of every Task-scope
   path. These forms are syntax errors outside a Task declaration or nested
   Task scope, with original UTF-8 byte spans.

2. **Direct Task targets.** `spawn` accepts only a direct application of a
   resolved Task declaration: `spawn target arguments`. Task declarations are
   not first-class values, ordinary function targets, constructors, or
   overload candidates. A Task declaration cannot be recursively spawned by
   the same lexical spawn chain in this initial frontend profile. Higher-order
   Task values, dynamic dispatch, implicit spawn, and Task declaration nesting
   are rejected before checked publication.

3. **Handles and observation.** `spawn task-call` yields an internal
   `TaskHandle<T, E>` whose result type `T` and latent Effect Row `E` come from
   the target declaration. `await handle` consumes that exact handle and
   yields `T`. A handle is non-copyable, cannot be compared, formatted,
   returned, captured by a closure or Handler, stored in an aggregate or
   mutable Place, passed to an ordinary function, or moved outside its owning
   lexical scope. Every control-flow path must consume each handle exactly once
   before scope exit; zero or multiple observations are checked errors.

4. **`let!` desugaring.** `let! pattern = task-call` is checked as one fused
   spawn followed immediately by await and pattern binding. It creates the same
   spawn-site, suspension, cancellation, cleanup, and source identities as the
   explicit two-form sequence. The pattern follows existing irrefutability and
   type rules. Desugaring preserves the original `let!`, target, pattern, and
   call spans for diagnostics and Audit Source.

5. **Lexical ownership.** Each `scope` owns a stable `ScopeId`; each syntactic
   spawn site owns a stable `TaskId`, `CancellationId`, and `CleanupId`; and
   each `await`/`let!` owns a stable `SuspensionId`. A spawned child has exactly
   one lexical parent spawn site and one owning scope. Handles may be awaited
   only in that scope. Nested scopes cannot observe an outer handle, and outer
   scopes cannot observe a nested handle. `detach` is not accepted syntax in
   this profile.

6. **Types and Effects.** A Task declaration has a non-first-class checked
   signature `(P...) -> Task<T, E>`, where `E` is its checked body Effect Row.
   The source type grammar does not expose `Task`, `TaskHandle`, or their
   constructors in this slice. `spawn` contributes the target's latent `E`
   plus the unhandleable structural label `Task.Spawn`; `await` contributes
   `Task.Await`; fused `let!` contributes both. The structural labels require
   no host Capability, cannot be declared in `requires`, cannot be handled by
   RFC-0006 Handler clauses, and remain visible in checked Effect Rows.

7. **Suspension safety.** At each suspension, the checker records the exact
   live binding identities and types. A mutable-place borrow, open mutation
   operation, Handler continuation, Task handle other than the awaited handle,
   or unresolved Trait obligation may not cross the suspension. Ordinary owned
   immutable values and accepted shared Cell identities may remain live. This
   is a static rule only; it does not define frame layout or runtime resumption.

8. **Checked publication.** Positive syntax lowers through normal AST, HIR,
   resolution, type, and Effect checking into an immutable `CheckedTaskCore`
   owned by `CheckedProgram`. It contains the Task definition identity,
   signature, root scope, canonical scope/spawn parent graph, suspension live
   sets, cancellation and cleanup identities, result body, original source
   spans, and the accepted DEC-0091 `TaskCore` projection. Construction fails
   atomically on an unresolved identity, duplicate identity, cycle, escaped or
   misused handle, missing return, invalid suspension live set, or Effect/type
   disagreement. Evaluation must consume this checked representation only.

9. **Identity and presentation.** IDs derive from existing canonical lexical
   owner/body identity plus source-order ordinal; physical paths, allocation
   order, hash-map order, and debug text are excluded. Equivalent reconstruction
   is byte deterministic. Semantic Graph represents a Task declaration as a
   definition with an explicit Task signature and checked scope/spawn/
   suspension extension; existing node meanings do not change. Audit Source
   receives a new Experimental revision that round-trips the exact accepted
   Task syntax while preserving Author Source spans. Exact schema/version
   registration is part of TASK-2201 implementation and cannot claim Stable.

10. **Non-executable boundary.** Until TASK-2202 and TASK-2203 have Accepted
    authority, `check`, Semantic Graph, and Audit Source may publish a valid
    checked Task program, but `run`, `test`, bytecode lowering, VM execution,
    REPL submission, and project artifacts must reject it before evaluation
    with a registered bilingual implementation-boundary diagnostic. No AST/HIR
    interpretation or placeholder runtime API is permitted.

## Conformance plan

- Parse and lower direct Task declarations, nested scopes, explicit
  spawn/await, and fused `let!` with ASCII and Chinese identifiers; freeze CST,
  AST, HIR, checked Task Core, Semantic Graph, and Audit round-trip evidence.
- Reject Task forms outside Task scope, missing/non-final return, first-class or
  indirect targets, recursive spawn chains, invalid arity/type/effects,
  handle escape/copy/double-await/unobserved paths, cross-scope observation,
  unsupported detach, and invalid live values before checked publication.
- Cover conditionals and exhaustive matches where every path observes the same
  handles exactly once; reject path-sensitive leaks and duplicate observation.
- Verify canonical scope/task/suspension/cancellation/cleanup identities,
  insertion-order-independent bytes, alpha-renaming rules, and physical-path
  independence.
- Preserve exact BOM/CRLF/Unicode 17.0.0 byte spans, bilingual registered
  diagnostics, deterministic Semantic IDs, and versioned Audit fixtures.
- Prove `run`, `test`, REPL, project build, bytecode, interpreter, and VM reject
  checked Task programs before execution while ordinary Seed and Handler
  programs retain byte-identical behavior.

## Compatibility impact

- Adds Experimental contextual syntax and checked Task metadata; existing
  identifiers named `task`, `scope`, `spawn`, `await`, or `return` remain valid
  outside the exact contextual grammar positions.
- Adds internal Task/TaskHandle types and `Task.Spawn`/`Task.Await` checked
  labels. They are not source-constructible, handler-interceptable, host
  capabilities, or runtime values in this slice.
- Requires new bilingual Task-checking diagnostics and an Experimental Audit
  revision during implementation. Existing diagnostic meanings, Semantic
  Graph/Audit revisions, Program IDs for non-Task programs, bytecode 1.0–1.4,
  CLI success behavior, packages, LSP, Native/Wasm, and Unicode 17.0.0 remain
  unchanged.
- Adds no executable Task promise, scheduler order, ABI, bytecode instruction,
  public replay protocol, detach authority, deadline, or Stable compatibility.

## Unresolved alternatives

- Implicit eager Task calls, first-class Task values, globally reserved
  keywords, automatic observation of non-Unit results, cross-scope handles,
  recursive spawn, Handler interception of Task structural labels, and detach
  are rejected from the initial frontend profile.
- State-machine encoding, frame liveness layout, runtime join/cancel/cleanup,
  child Fault aggregation, deterministic test scheduling, production worker
  scheduling, resource limits, stress behavior, Actor crossing, deadlines,
  Replay, Native/Wasm, and migration remain TASK-2202 through TASK-2206 work
  requiring separate Accepted authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
