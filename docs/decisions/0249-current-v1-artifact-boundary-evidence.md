# DEC-0249: Current v1 artifact boundary evidence / 当前 v1 发布物边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role：release engineering
> 相关 RFC/缺口：DEC-0056 | DEC-0248 | DEC-0242 | RC-6905
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes composing the current RC2→RC3→RC1→RC0 inventory
chain and correcting LSP/protocol facts in the v1 artifact inventory. It does
not authorize or publish a v1 artifact.

本决定授权 v1 发布物盘点组合当前 RC2→RC3→RC1→RC0 链并修正 LSP/协议事实；它不授权或发布任何 v1 发布物。

## Question

How should the v1 publication inventory acknowledge the source-built Preview
LSP server and 27 protocols while preserving the absence of distributable,
Stable release artifacts?

## Decision

1. Amend the bounded DEC-0056 gate by calling
   `rc2_change_control::check_repository`; RC2 composes RC3→RC1→RC0.
2. Correct the language-server row: source-built `ling lsp --stdio` exists,
   but signed binaries, acquisition, editor discovery, document features,
   restart evidence, and Stable distribution do not.
3. Correct the protocol row from 21 to 27 records while retaining zero Stable
   protocols and Future `PROTO-EVIDENCE`.
4. Require upstream-pass and four-parent-blocked markers and fail closed with
   internal `GOV-V1-ARTIFACT-0011` errors on current-evidence drift.
5. Keep all fourteen release-item states unchanged and retain every artifact,
   signature, SBOM/provenance, documentation, support, migration, security,
   conformance, and evidence-bundle exit.
6. The verifier remains deterministic, read-only, and offline and creates no
   tag, artifact, signature, upload, install, network request, or system change.

## Conformance plan

- Run `cargo xtask v1 verify` and require fourteen items, nine audits, and one
  composed upstream gate.
- Run RC2, RC3, RC1, RC0, protocol, LSP-discovery, and Zed gates independently.
- Supply stale no-LSP/21-protocol/upstream facts in a focused test and require
  fail-closed internal governance errors.
- Run workspace, CI, governance, support, status, traceability, Clippy,
  formatting, deterministic, and offline gates.

## Compatibility impact

Documentation correction and internal evidence composition only. Ling syntax,
semantics, diagnostics, schemas, Semantic IDs, packages, dependencies,
CLI/LSP/DAP/runtime behavior, Unicode 17.0.0, protocol states, support states,
and public APIs are unchanged. No migration is required.

## Unresolved alternatives

v1 candidate/tag; compiler/runtime packaging; checksums/signatures;
SBOM/provenance; standard-library publication; Zed/LSP distribution; complete
manuals and migration; final support; Stable protocol corpus; Tier1
conformance; security policy; evidence bundle; and publication remain deferred.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
