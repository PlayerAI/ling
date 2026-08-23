# DEC-0242: Current LSP discovery boundary / 当前 LSP 发现边界

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role：editor integration
> 相关 RFC/缺口：RFC-0004 | DEC-0049 | ZED-6802
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes correcting discovery evidence to acknowledge the
implemented source-built `ling lsp --stdio` entry point while keeping editor
discovery and acquisition unimplemented.

本决定授权修正发现证据：承认源码构建的 `ling lsp --stdio` 入口已经实现，同时继续明确编辑器发现与获取尚未实现。

## Question

How should the discovery inventory stop claiming that the Ling CLI is not an
LSP server without converting a source-built Preview entry point into an
accepted Zed discovery, distribution, or installation contract?

## Decision

1. Record `ling lsp --stdio` as an implemented, process-tested Preview server
   entry point adjacent to discovery/acquisition.
2. Record PATH lookup as `Not established`, not `Unavailable`: a source-built
   executable exists, but no extension lookup, accepted executable identity,
   compatibility/version check, or bounded process-start policy exists.
3. Keep user configuration and failure/install guidance `Not established` and
   official release download `Unavailable`.
4. Extend `cargo xtask lsp verify` to bind the current claim to six repository
   files: workspace membership, CLI dependency, CLI dispatch, CLI process test,
   LSP crate identity, and the registered Preview lifecycle protocol.
5. Parse Cargo and protocol TOML structurally, and reject missing workspace
   members, incorrect dependency paths, wrong crate identity, lifecycle
   version/stability/implementation drift, missing producer identity, or
   missing CLI/test wiring.
6. The verifier remains read-only and offline. It does not search PATH, read
   settings, start a discovery candidate, contact a registry, download,
   install, allocate a diagnostic, or define editor behavior.
7. Parent `ZED-6802` remains `BlockedSpec` until accepted discovery/provenance
   authority and executable editor fixtures exist.

## Conformance plan

- Verify four exact priority rows: three `Not established` and one
  `Unavailable`.
- Structurally parse current Cargo manifests and the protocol inventory, then
  validate exact LSP crate, dependency, lifecycle, stability, implementation,
  and producer fields.
- Validate the CLI dispatch and real-process lifecycle test markers.
- Mutate workspace membership, dependency identity, lifecycle identity, or CLI
  wiring in a focused unit test and require fail-closed internal diagnostics.
- Run LSP, CLI process, workspace, CI, governance, status, Clippy, formatting,
  deterministic, and offline gates.

## Compatibility impact

Documentation correction and stronger internal evidence validation only. Ling
semantics, source syntax, diagnostics, public schemas, Semantic IDs, packages,
dependencies, CLI/LSP behavior, runtime, Unicode 17.0.0, protocols, support
states, and editor APIs are unchanged. No migration is required.

No setting, PATH search, standalone binary, release URL, checksum/signature
record, installer, public error schema, Zed extension, network behavior, or
Stable support claim is added.

## Unresolved alternatives

Executable identity and version negotiation; user/workspace configuration;
PATH precedence; signed release artifacts; download trust and atomic install;
offline fallback; bilingual installation diagnostics; Zed packaging and tests;
cross-host evidence; and G6 sign-off remain deferred.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
