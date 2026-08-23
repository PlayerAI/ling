# `ling.lsp.workspace/0.1` Experimental fixture

This fixture records the Accepted RFC-0030 atomic workspace-reload boundary:

- initialize advertises the exact version and source/input/byte limits;
- `ling/workspace/reload` is a Ready-state request, never a mutating
  notification;
- canonical-decimal base revisions reject stale publication without guessing;
- unique workspace/dependency source and manifest/lock/config/profile/target
  deltas are canonicalized independently of client array order;
- exact UTF-8 disk/input bytes, BOM, CRLF, and Unicode are preserved;
- open overlays hide disk upserts and cannot be removed by reload;
- a failed later delta leaves visible bytes, disk bytes, inputs, document
  versions, version history, and the VFS revision unchanged;
- successful batches publish once and no-op batches retain their revision;
- watcher timing, paths, symlinks, filesystem reads, diagnostics, and compiler
  result publication remain outside this fixture.

Executable evidence:

```text
cargo test -p ling-source --all-targets --locked --offline
cargo test -p ling-db --all-targets --locked --offline
cargo test -p ling-lsp --test workspace_reload --locked --offline
```
