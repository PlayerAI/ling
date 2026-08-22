# DEC-0031: LSP internal cooperative cancellation boundary / LSP 内部协作取消边界

> 状态：Accepted
> Status: Accepted
> Proposed date: 2026-08-22
> Decision date: 2026-08-22
> Owner role: ide-protocol-design
> Related authority/gap: `DEC-0019`, `GAP-LSP-TRANSACTION-PROTOCOL-001`
> Lifecycle record: `docs/governance/lifecycle.toml`

This decision fixes only a small in-process cancellation primitive for future
LSP analysis work. It does not define JSON-RPC `$/cancelRequest`, request IDs,
compiler query propagation, result publication, or a public error protocol.

## Question

Future analysis tasks need a shared, deterministic way for an owner to signal
cooperative cancellation to a worker without reusing VM host control or
publishing a partial result. DEC-0019 permits an internal cooperative query
checkpoint but explicitly leaves compiler-facing and LSP request cancellation
to a separate decision.

## Decision

1. `ling_lsp::CancellationToken` is a cloneable, in-process token backed by one
   monotonic cancellation bit. Any clone may request cancellation, and every
   clone observes the same state.
2. `cancel` is idempotent. `is_cancelled` is a non-blocking observation, and
   `check` returns the typed `CancellationError::Cancelled` checkpoint result
   or success. The primitive does not sleep, poll wall-clock time, or spawn a
   thread.
3. The token is an analysis utility only. It carries no request ID, document
   version, snapshot identity, deadline, priority, diagnostic code, JSON-RPC
   response, or Workspace Edit state. A caller must check it before publishing
   any result and must discard its own partial work when cancellation is
   observed.
4. The token is deliberately separate from RFC-0020's VM
   `CancellationToken`; runtime cancellation and `execution.cancelled` faults
   must not become compiler/LSP compatibility behavior through reuse.
5. This decision adds no transport handler, scheduler, compiler query API, or
   public protocol inventory entry. It only provides the typed internal
   primitive and deterministic unit evidence.

## Conformance plan

- Verify a new token starts active, every clone observes cancellation, and
  repeated cancellation remains idempotent.
- Verify `check` succeeds before cancellation and returns the same typed error
  after cancellation, independent of clone/order or repeated observations.
- Verify no thread, timing, request ID, document version, VM fault, diagnostic,
  or JSON serialization is involved in the primitive.

## Compatibility impact

- Adds an internal `ling-lsp` Rust value only; language syntax, semantics,
  diagnostics, schemas, Semantic IDs, source spans, CLI, LSP wire methods,
  bytecode, VM, and Unicode 17.0.0 remain unchanged.
- Preserves DEC-0019's cooperative checkpoint boundary and keeps RFC-0020 VM
  cancellation independent.
- The token is not a public wire protocol and is absent from the protocol
  inventory and support matrix.

## Unresolved alternatives

JSON-RPC request cancellation, request-ID lifetime, snapshot association,
compiler/query propagation, cancellation-versus-completion precedence,
partial-result suppression, deadlines, fairness, resource limits, and
diagnostic/migration behavior require later accepted authority.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
