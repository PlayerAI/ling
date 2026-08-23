# DEC-0246: Current RC1 boundary evidence / 当前 RC1 边界证据

> 状态：Accepted
> 提出日期：2026-08-23
> 决定日期：2026-08-23
> Owner role：release engineering
> 相关 RFC/缺口：DEC-0053 | DEC-0245 | DEC-0243 | RC-6902
> 生命周期记录：`docs/governance/lifecycle.toml`

This decision authorizes correcting the RC1 Zed evidence and composing the
current RC0 and Zed acceptance gates. It does not authorize public validation,
a Zed extension, or any release exit.

本决定授权修正 RC1 的 Zed 证据，并组合当前 RC0 与 Zed 验收门禁；它不授权公开验证、Zed 扩展或任何发布退出条件。

## Question

How should the RC1 inventory acknowledge the verified source-built Preview
LSP server while preserving the accurate conclusion that no Zed extension or
public RC1 validation surface exists?

## Decision

1. Amend the bounded DEC-0053 inventory gate by composing
   `rc0_freeze::check_repository` and `zed_extension::check_repository` from
   `cargo xtask rc1 verify`.
2. Correct the Zed row to state that source-built `ling lsp --stdio` and its
   current grammar/LSP/position prerequisites exist.
3. Keep the Zed-extension row `Unsupported`: no Zed manifest, extension
   package, acquisition flow, marketplace artifact, or debugger integration
   exists.
4. Require explicit current-evidence and RC0-blocked markers and fail closed
   with internal `GOV-RC1-VALIDATION-0011` errors on drift.
5. Keep all nine RC1 criterion states unchanged and retain every required
   public-validation exit and no-publication guardrail.
6. The composed verifier is deterministic, read-only, and offline and creates
   no artifact, package, install, signature, issue, migration, network, or
   system mutation.

## Conformance plan

- Run `cargo xtask rc1 verify` and require nine criteria, eight audit files,
  and two current-evidence gates.
- Run the RC0 and Zed-extension verifiers independently.
- Remove the current LSP/Zed/RC0 markers in a focused test and require a
  fail-closed internal governance error.
- Run workspace, CI, governance, support, status, traceability, Clippy,
  formatting, deterministic, and offline gates.

## Compatibility impact

Documentation correction and internal evidence composition only. Ling syntax,
semantics, diagnostics, public schemas, Semantic IDs, packages, dependencies,
CLI/LSP/DAP behavior, runtime, Unicode 17.0.0, protocol states, support states,
and public APIs are unchanged. No migration is required.

## Unresolved alternatives

RC0 completion; public artifacts and acquisition; checksums, SBOM, provenance,
and signing; clean install; Zed packaging/marketplace; sample manifests;
migration; issue intake; schema-reset change control; independent validation;
and public RC1 approval remain deferred.

## Supersession

- Supersedes: `None`
- Superseded by: `None`
