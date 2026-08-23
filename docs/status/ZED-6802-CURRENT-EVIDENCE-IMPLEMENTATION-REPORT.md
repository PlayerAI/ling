# ZED-6802-CURRENT-EVIDENCE Implementation Report

## Result

The discovery inventory now acknowledges the implemented, process-tested
Preview `ling lsp --stdio` entry point while proving that editor discovery and
acquisition remain absent. The PATH row is `Not established`; it is no longer
based on the false premise that no LSP-capable Ling executable exists.

The parent `ZED-6802` remains `BlockedSpec`. No acquisition behavior or Zed
support is claimed.

## Implementation

- `docs/testing/LSP-DISCOVERY-ACQUISITION.md` separates the source-built server
  entry point from user settings, PATH discovery, released downloads, and
  installation guidance.
- `tools/xtask/src/lsp_discovery.rs` validates four exact priority rows and six
  current evidence files. Cargo manifests and the protocol inventory are
  parsed as TOML; CLI dispatch and the real process test are checked as current
  Rust evidence.
- A focused negative test rejects missing workspace membership, a wrong CLI
  dependency, a wrong LSP crate identity, absent lifecycle registration, and
  missing CLI wiring.
- Authority and status documents retain every network, process, provenance,
  diagnostic, packaging, and editor-integration blocker.

## Verification scope

The focused verifier and CLI/LSP tests prove only the existing source-built
Preview server and the accuracy of the negative acquisition boundary. Full
repository gates provide offline, deterministic, governance, formatting, and
lint evidence for this change.

No Zed process, PATH lookup, configuration lookup, release download,
installation, network request, or cross-host editor integration is executed.

## Compatibility and deferrals

No Ling or editor behavior changes. Accepted discovery settings and executable
identity, process policy, signed release artifacts, secure download/install,
offline fallback, bilingual public diagnostics, per-platform and Zed fixtures,
and G6 sign-off remain deferred.
