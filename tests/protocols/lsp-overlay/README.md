# `ling.lsp.overlay/0.2` Experimental fixture

This fixture records the RFC-0023 full-text surface plus RFC-0029 bounded
incremental synchronization implemented by `crates/ling-lsp`:

- `ling://workspace/<path>.ling` is writable;
- `ling://dependency/<package>/<path>.ling` is readable and read-only;
- `untitled://ling/<path>.ling` is writable and removed on close;
- `didOpen`, `didChange`, and `didClose` preserve lifecycle and notification
  rules;
- a change batch contains 1–64 ordered full or ranged replacements;
- ranges use the initialize-negotiated UTF-8, UTF-16, or UTF-32 encoding and
  are projected only through the shared `SourceMap` API;
- each entry observes preceding edits, and each range rebuilds the source map
  and line index for its current snapshot;
- the VFS and client version are published once after the complete batch;
- accepted document versions are strictly monotonic;
- an open overlay hides disk updates until close;
- invalid URI, stale-version, read-only, malformed, boundary, size, and failed
  later-edit cases leave visible bytes, version, and VFS revision unchanged;
- the valid version-0.1 single-full-replacement shape remains accepted,
  including exact UTF-8 editor buffers that are not yet valid Ling source.

Executable evidence:

```text
cargo test -p ling-lsp --test overlay --locked --offline
cargo test -p ling-lsp --test incremental_changes --locked --offline
```

Compiler request snapshots, stale analysis results, diagnostics, cancellation,
Workspace Edits, Semantic Transactions, and Stable compatibility remain outside
this fixture by design.
