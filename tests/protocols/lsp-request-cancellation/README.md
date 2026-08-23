# `ling.lsp.request-cancellation/0.1` Preview fixtures

Accepted RFC-0049 defines the bounded stdio cancellation contract:

- exact string and number request IDs receive one live cooperative token;
- standard notification-form `$/cancelRequest` signals only the matching live
  request, while unknown, duplicate, malformed, and late notifications are
  response-free no-ops;
- request-form cancellation is rejected with Invalid Request `-32600`;
- observed cancellation returns RequestCancelled `-32800` and publishes no
  partial response, Workspace Edit, completion-resolve batch, workspace index,
  semantic-token history, diagnostic, or compiler cache entry;
- compiler-backed queries check bounded stages and return typed cancellation
  without inserting partial checked results.

Executable evidence:

- `fixtures/v1.json` freezes exact envelope behavior and initialize discovery
  under the test-only `ling.test.lsp-request-cancellation/1` format.
- `crates/ling-lsp/tests/cancellation.rs` reads every fixture and also executes
  a deterministic framed transcript in which the reader cancels a live queued
  workspace-symbol request while the single server executor is blocked on an
  earlier response.
- `crates/ling-lsp/tests/completion.rs`, `rename.rs`, `semantic_tokens.rs`, and
  `workspace_symbols.rs` cover method-specific cancellation and atomic
  publication; `ling-db` and `ling-types` unit tests cover typed query/solver
  cancellation and the no-partial-cache boundary.

Command:

```text
cargo test -p ling-lsp --test cancellation --locked --offline
```

The fixture format is test-only, not a public schema. Incompatible public ID,
method, precedence, discovery, error, or publication behavior requires a new
Accepted protocol marker and migration evidence.
