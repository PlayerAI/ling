# `ling.lsp.overlay/0.1` Preview fixture

This fixture records the bounded RFC-0023 surface implemented by
`crates/ling-lsp`:

- `ling://workspace/<path>.ling` is writable;
- `ling://dependency/<package>/<path>.ling` is readable and read-only;
- `untitled://ling/<path>.ling` is writable and removed on close;
- `didOpen`, `didChange`, and `didClose` accept full-text synchronization only;
- accepted document versions are strictly monotonic;
- an open overlay hides disk updates until close;
- invalid URI, stale-version, read-only, and ranged-change cases leave the VFS
  unchanged.

The executable evidence is `cargo test -p ling-lsp --test overlay --locked
--offline`. Incremental edits, diagnostics, compiler snapshots, Workspace
Edits, and Semantic Transactions remain outside this fixture by design.
