# ZED-6802 Authority Audit

- Task: `ZED-6802` — Language-server discovery/acquisition
- Plan: `docs/ling_execution_plan/10-G6-V1.0-STABILIZATION.md:442-459`
- Release: G6
- Status: `BlockedSpec`; bounded current-evidence work is accepted separately.

## Decision

`ZED-6802` remains `BlockedSpec`. There is no dedicated discovery/acquisition
implementation, distributed language-server release artifact, Zed extension,
accepted discovery key, PATH contract, download manifest, checksum/signature
registry, or public installation/error protocol. The source-built Ling CLI
does implement and process-test the Preview `ling lsp --stdio` entry point, but
that entry point is not an acquisition source. The inventory records each
planned source as `Unavailable` or `Not established` and states the security
rules that a future Accepted decision must fix before any network or discovery
process operation exists.

The compiler CLI and Tree-sitter grammar are not treated as language-server
fallbacks. No setting, URL, schema, diagnostic, or installer is created merely
to make the plan appear implemented.

## Normative traceability

- `10-G6-V1.0-STABILIZATION.md:442-459` is a non-normative checklist. It does
  not authorize a setting key, executable identity, download source, or
  protocol.
- `docs/ling_execution_plan/04-LSP-IMPLEMENTATION.md` and
  `05-ZED-EXTENSION.md` are planning inputs; their future server and download
  sketches do not outrank accepted specifications or create current APIs.
- `docs/governance/support-matrix.toml` records `UNSUP-LSP-EDITOR` for document
  features and editor integration; the protocol inventory records the
  implemented Preview lifecycle and no discovery/acquisition protocol.
- `docs/ROADMAP-1.0.md` requires accepted public protocols, deterministic and
  offline behavior, security evidence, and release artifacts before Stable
  support.
- `AGENTS.md` requires bilingual registered diagnostics, Unicode 17.0.0,
  original UTF-8 spans, no arbitrary placeholder APIs, and explicit
  unsupported/future boundaries.

## Evidence and gaps

`docs/testing/LSP-DISCOVERY-ACQUISITION.md` maps user override, PATH, official
release, and failure guidance to the current repository state. It specifies
future HTTPS, version selection, checksum/signature, atomic-install,
no-execution-before-verification, override, offline, redaction, and bounded
process requirements without claiming any of them are implemented.

The internal `cargo xtask lsp verify` command protects the four-row inventory,
binds the adjacent Preview CLI/server facts to six current repository files,
and rejects state drift, false implementation evidence, or stale legacy names.
It does not create a public LSP setting, installer, download URL, standalone
executable, or discovery protocol.

Accepted `DEC-0242` closes only the bounded
`ZED-6802-CURRENT-EVIDENCE` child. It corrects the obsolete claim that the
compiler CLI is not an LSP server while retaining the absence of editor
discovery and acquisition.

The missing evidence is an Accepted discovery and provenance decision, a
versioned LSP binary/protocol, public diagnostics and install schema, secure
per-platform artifacts, and executable positive/negative/malformed/offline
fixtures. These remain G1-G5/G6 dependencies.

## Compatibility and deferred work

This audit changes no language semantics, diagnostics, schemas, Semantic IDs,
CLI commands, package behavior, runtime, editor protocol, dependencies, or
public API. It preserves `ling`/`.ling`, Unicode 17.0.0, original UTF-8 spans,
deterministic/offline Rust validation, and the explicit unsupported LSP/Zed
boundary.

No network request, installer, executable, download, cache, or system
configuration was changed. Discovery, acquisition, signature/trust policy,
offline fallback, and user-facing installation guidance remain deferred.
