# `ling.lsp.publish-diagnostics/0.2` Experimental fixture

This fixture records the Accepted RFC-0032 deterministic push-diagnostic
writer:

- successful state-changing open/change/close, disk publication, and workspace
  reload operations schedule one complete-state analysis; rejected and no-op
  operations do not;
- the explicit message-boundary debounce coalesces mutations, writes a caused
  request response first, and publishes only a full still-current snapshot;
- syntax diagnostics take precedence over complete non-temporary workspace
  HIR, resolution, type, and Effect diagnostics, while temporary documents are
  syntax-only;
- open results carry the exact client version, closed/disk results omit it,
  and source removal or temporary close emits deterministic empty clearance;
- exact unchanged entries are suppressed, changed entries are URI-sorted, and
  a stale, failed, unknown-URI, or oversized result leaves the ledger and
  pending work unchanged;
- RFC-0034 root control applies the discovered immutable caps, preserves every
  retained `ling.lsp.diagnostic/0.2` value byte-for-byte, and reports omissions
  with registered `L-LSP-0001` summaries;
- output uses the separately registered `ling.lsp.diagnostic/0.2` values and
  performs no filesystem, environment, registry, network, shell, or host-path
  access.

Executable evidence:

```text
cargo test -p ling-db --test workspace_diagnostics --locked --offline
cargo test -p ling-lsp --test diagnostic_adapter --locked --offline
cargo test -p ling-lsp --test push_diagnostics --locked --offline
```
