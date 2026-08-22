# DEC-0032: LSP internal deterministic work ordering boundary / LSP 内部确定性工作排序边界

> 状态：Accepted
> Status: Accepted
> Proposed date: 2026-08-22
> Decision date: 2026-08-22
> Owner role: ide-protocol-design
> Related authority/gap: `DEC-0021`, `GAP-LSP-TRANSACTION-PROTOCOL-001`
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision authorizes only a small in-process ordering primitive for future
LSP analysis work. It does not define debounce timing, JSON-RPC scheduling,
request freshness, result publication, or a public priority contract.

## Question

Future LSP analysis needs a deterministic way to order already-created work
items, but LSP-2503 has no accepted event, debounce, fairness, or publication
protocol. DEC-0021 permits deterministic internal scheduling for pure compiler
jobs, yet it does not define an LSP queue or authorize editor-visible behavior.

## Decision

1. `ling_lsp` may contain an internal `InternalWorkQueue` that stores opaque
   work identifiers and one of three logical priorities: `Interactive`,
   `Analysis`, or `Background`. The queue is `pub(crate)` and is not a wire or
   application-facing API.
2. Queue order is canonical: all `Interactive` items precede `Analysis`, all
   `Analysis` items precede `Background`, and items with the same priority are
   ordered by a monotonic enqueue sequence. The sequence is local to one queue
   instance and has no cross-process or serialized identity.
3. Enqueue and pop are pure in-process data operations. The queue does not
   spawn workers, sleep, observe wall-clock time, inspect host CPU state, or
   execute the work item. Sequence exhaustion returns an internal error rather
   than wrapping or changing order.
4. The queue does not coalesce duplicate identifiers and carries no document
   version, VFS revision, request ID, cancellation token, deadline, diagnostic,
   result, Workspace Edit, or Semantic Transaction state. Supersession,
   cancellation association, fairness, starvation bounds, and publication
   remain outside this child boundary.
5. `LspServer` and the stdio transport do not use this queue until an Accepted
   LSP-2503 authority defines event triggers, debounce/coalescing, freshness,
   cancellation, resource budgets, and publication. This child therefore
   changes no LSP method or capability.

## Conformance plan

- Verify canonical priority ordering and FIFO order within each priority.
- Verify repeated queues fed with the same logical sequence produce identical
  output independent of host timing or map insertion order.
- Verify empty/clear behavior, duplicate identifiers, and sequence exhaustion
  do not panic, wrap, or publish partial work.
- Verify the queue has no JSON serialization, transport handler, timer, worker,
  revision, request, diagnostic, or Workspace Edit surface.

## Compatibility impact

- Adds only `pub(crate)` Rust scheduling values in `ling-lsp`; language syntax,
  semantics, diagnostics, schemas, Semantic IDs, source spans, CLI, LSP wire
  methods, bytecode, VM, and Unicode 17.0.0 behavior remain unchanged.
- Does not alter DEC-0021 compiler scheduling, cache formats, process exit
  behavior, protocol inventory, support matrix, or migration requirements.
- The strict ordering is an internal testable data-structure contract, not a
  promise about editor latency, host scheduling, or public request priority.

## Unresolved alternatives

Debounce intervals, logical versus wall-clock batching, event triggers,
priority fairness, starvation limits, worker budgets, dependency expansion,
revision supersession, cancellation/result precedence, diagnostic replacement,
progress, and Stable versus Experimental protocol lifecycle require a later
Accepted decision for the parent LSP-2503 task.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
