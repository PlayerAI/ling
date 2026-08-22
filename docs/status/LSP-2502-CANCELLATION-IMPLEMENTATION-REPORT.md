# LSP-2502-CANCELLATION Implementation Report: Internal Cooperative Token

## Status

`Done` for the bounded internal child authorized by Accepted DEC-0031. The
parent LSP-2502 task remains `BlockedSpec` for public JSON-RPC/compiler
cancellation and result publication.

## Normative clauses covered

- DEC-0031 §1–§2: `CancellationToken` is clone-shared, monotonic, idempotent,
  non-blocking, and exposes deterministic `is_cancelled`/`check` operations.
- DEC-0031 §3: the token carries no request ID, document version, snapshot
  identity, deadline, diagnostic, JSON-RPC response, or Workspace Edit state;
  callers retain responsibility for discarding partial work.
- DEC-0031 §4–§5: the token is separate from RFC-0020 VM cancellation and adds
  no transport, scheduler, compiler query, or protocol inventory behavior.
- DEC-0019 remains the authority for the internal cooperative checkpoint
  boundary.

## Implementation

- `crates/ling-lsp/src/lib.rs` adds `CancellationToken` and the typed
  `CancellationError::Cancelled` checkpoint result using a shared atomic bit.
- `crates/ling-lsp/tests/cancellation.rs` verifies clone propagation,
  idempotent cancellation, pre/post-cancel checks, and independent token
  isolation.

No `$/cancelRequest` handler, request-ID state, compiler cancellation API,
partial-result publisher, diagnostic allocation, or public protocol was added.

## Verification

```text
cargo test -p ling-lsp --test cancellation --locked --offline
cargo clippy -p ling-lsp --all-targets --all-features --locked --offline -- -D warnings
```

Both commands pass. Full workspace and governance gates are required before
the completion hash is recorded.

## Compatibility and determinism

Cancellation is a local atomic observation with no timing, thread, host path,
VM fault, language, diagnostic, schema, Semantic ID, source-span, bytecode,
Unicode, CLI, or LSP wire effect. Repeated calls and clone/order variations
produce the same result.

## Deferred work

Public JSON-RPC cancellation, request IDs, snapshot association,
compiler/query propagation, cancellation-versus-completion precedence,
partial-result suppression, deadlines, fairness, resource limits, and
diagnostic/migration behavior remain in parent LSP-2502.
